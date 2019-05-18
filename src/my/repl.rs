extern crate regex;

use std::path::{ PathBuf, Path };
use std::io::{ stdin, stdout, Write};
use std::str::FromStr;
use self::regex::Regex;

use my::user::*;
use my::command::*;

const SHOW:   &'static str = "show";
const LOGOFF: &'static str = "logoff";


pub fn begin_loop(amount: usize, root_folder: &Path, resources: &[PathBuf]) {
    let mut users = create_users(amount, resources);
    let root = User::root(resources);
    users.push(root);
    loop {
        print!("Login: ");
        stdout().flush().unwrap();
        let mut login = String::new();
        stdin().read_line(&mut login).expect("Enter a valid login string!");

        print!("Password: ");
        stdout().flush().unwrap();
        let mut password = String::new();
        stdin().read_line(&mut password).expect("Enter a valid password string!");

        let login_result = do_login(&mut users, &login.trim(), &password.trim());

        match login_result {
            Err(error_string) => println!("{}", error_string),
            Ok(user_id)       => {
                println!("User logged in successfully.");
                accept_inputs(root_folder, &mut users, user_id);
            }
        }
    }
}

fn accept_inputs(root_folder: &Path, users: &mut [User], current_id: usize) {
    let read_regex  = Regex::new(r"read\s+(?P<resource>[a-zA-z0-9\\.-]*)").unwrap();
    let write_regex = Regex::new(r"^write\s+(?P<resource>[a-zA-z0-9\\.-]*)\s+(?P<text>[a-zA-z0-9\\.-]*)").unwrap();
    let grant_regex = Regex::new(r"^grant\s+(?P<privilege>[a-zA-z0-9\\]*)\s+(?P<resource>[a-zA-z0-9\\.-]*)\s+(?P<user>[a-zA-z0-9\\]*)").unwrap();

    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut cmd = String::new();
        stdin().read_line(&mut cmd).expect("Enter a valid command string!");
        let cmd = cmd.trim();
        if cmd == SHOW {
            let current: &mut User = &mut users[current_id];
            do_ls(root_folder, &current);
        } else if read_regex.is_match(&cmd) {
            let it = read_regex.captures(&cmd).unwrap();
            let current: &mut User = &mut users[current_id];
            do_read(&current, &it["resource"]);
        } else if write_regex.is_match(&cmd) {
            let it = write_regex.captures(&cmd).unwrap();
            let rs = &it["resource"];
            let text = &it["text"];
            let current: &mut User = &mut users[current_id];
            do_write(&current, rs, text);
        } else if grant_regex.is_match(&cmd) {
            let it = grant_regex.captures(&cmd).unwrap();
            let pr = &it["privilege"];
            let rs = &it["resource"];
            let us = &it["user"];
            let id: Option<usize>;
            {
                match users.iter().enumerate()
                            .find(|ref u| u.1.name == us)
                            .map(|pair| pair.0) {
                                Some(i) => id = Some(i),
                                None    => id = None
                            }
            }
            if id.is_some() {
                let p = Privilege::from_str(pr);
                if p.is_ok() {
                    do_grant(users, current_id, id.unwrap(), p.unwrap(), rs);
                }
            }
        } else if cmd == LOGOFF {
            break;
        }
    }
}

fn create_users(amount: usize, resources: &[PathBuf]) -> Vec<User> {
    let admin = User::root(resources);
    let mut users = Vec::<User>::with_capacity((amount + 1) as usize);
    users.push(admin);
    for id in 0 .. amount {
        let u = User::ordinary(
            format!("user{}", id).to_string(),
            "123".to_string(),
            resources);
        users.push(u);
    }

    users
}

fn do_login(users: &mut[User], login: &str, pass: &str) -> Result<usize, String> {
    let user_opt = users.iter().enumerate().find(|ref us| us.1.name == login);
    match user_opt {
        None    => Err(format!("No user is registered under name {}", login)),
        Some((id, user)) => {
            if user.password == pass {
                Ok(id)
            } else {
                Err("Wrong credentials!".to_string())
            }
        }
    }
}

fn do_ls(folder: &Path, user: &User) {
    let filenames  = list_files(folder);
    let rights = &user.privileges;

    for file in filenames {
        let permission_str = 
        if rights.contains_key(&file) {
            rights.get(&file).unwrap_or(&vec![]).iter()
                    .map(|perm| match *perm {
                        Privilege::READ  => "r",
                        Privilege::WRITE => "w",
                        Privilege::SHARE => "s"
                    })
                    .fold(String::new(), |a, x| a + x)
        } else { "---".to_string() };
    
        println!("{}  {}", file, permission_str);
    }
}

fn do_read(user: &User, resource: &str) {
    if user.can_do(resource, &Privilege::READ) {
        match read_file(Path::new(resource)) {
            Ok(content) => println!("{}", content),
            Err(error)  => println!("Error occured: {}", error)
        }
    } else {
        println!("Access not allowed!");
    }
}

fn do_write(user: &User, resource: &str, content: &str) {
    if user.can_do(resource, &Privilege::WRITE) {
        match write_to_file(Path::new(resource), content) {
            Ok(()) => println!("File updated."),
            Err(e) => println!("Error writing to file: {}", e)
        }
    } else {
        println!("Access not allowed!");
    }
}

fn do_grant(users: &mut[User], who: usize, grant_to: usize, what: Privilege, on_what: &str) {
    if who == grant_to{
        println!("Cannot grant to yourself!");
    } else if users[who].can_do(on_what, &Privilege::SHARE) {
        users[grant_to].grant_permissions(on_what, &[what]);
    } else {
        println!("Access not allowed!");
    }
}