use std::fs::*;
use std::path::*;
use std::env::current_dir;

pub fn init_resource_folder(name: &str) -> Box<PathBuf> {
    let path = Path::new(name);
    if !path.exists() {
        match create_dir(name) {
            Ok(())      => println!("Dir {} successfully created.", name),
            Err(error)  => panic!("{:?}", error)
        };

        let mut path_buf = current_dir().unwrap();
        path_buf.push(name);
        Box::new(path_buf)
    } else {
        Box::new(path.to_path_buf())
    }
}

pub fn init_resources(amount: usize, folder: &Path) -> Vec<PathBuf> {
    let mut created_files: Vec<PathBuf> = Vec::with_capacity(amount);
    for i in 0 .. amount {
        let mut file_path: PathBuf = folder.to_path_buf();
        file_path.push(format!("file{}", i));
        if file_path.exists() {
            created_files.push(file_path);
        } else {
            match File::create(&file_path) {
                Ok(_) => {
                    created_files.push(file_path);
                },
                Err(error) => println!("An error occured: {}", error)
            };
        }
    }
    return created_files;
}