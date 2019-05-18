use std::path::*;
use std::collections::hash_map::HashMap;
use std::str::FromStr;

const ROOT_NAME: &'static str = "root";
const ROOT_PASS: &'static str = "root";

pub type Rights = HashMap<String, Vec<Privilege>>;

#[derive(Clone, PartialEq, Debug)]
pub enum Privilege {
    READ, WRITE, SHARE
}

impl FromStr for Privilege {
    type Err = ();
    fn from_str(s: &str) -> Result<Privilege, ()> {
        match s {
            "read"  | "READ"  => Ok(Privilege::READ),
            "write" | "WRITE" => Ok(Privilege::WRITE),
            "share" | "SHARE" => Ok(Privilege::SHARE),
            _ => Err(())
        }
    }
}

pub struct User {
    pub name: String,
    pub password: String,
    pub privileges: Rights
}

impl User {
    pub fn root(resources: &[PathBuf]) -> User {
        let mut root = User::new(ROOT_NAME.to_string(), ROOT_PASS.to_string());
        root.privileges = with_privilege(resources,
                                         &[Privilege::READ, Privilege::WRITE, Privilege::SHARE]);
        return root;
    }

    pub fn ordinary(name: String, password: String, resources: &[PathBuf]) -> User {
        let mut user = User::new(name, password);
        user.privileges = with_privilege(resources, &[Privilege::READ]);
        return user;
    }

    fn new(name: String, password: String) -> User {
        User { name : name, password: password, privileges : HashMap::new() }
    }

    pub fn grant_permissions(&mut self, resource: &str, permissions: &[Privilege]) {
        let rights: &mut Rights = &mut self.privileges;
        if rights.contains_key(resource) {
            rights.get_mut(resource).unwrap().extend(permissions.iter().cloned());
        } else {
            rights.insert(resource.to_string(), permissions.to_vec());
        }
    }

    #[allow (dead_code)]
    pub fn strip_permissions(&mut self, resource: &str, permissions: &[Privilege]) {
        let rights: &mut Rights = &mut self.privileges;
        if rights.contains_key(resource) {
            let arr: &mut Vec<Privilege> = rights.get_mut(resource).unwrap();
            arr.iter()
                .position(|ref s| permissions.contains(s))
                .map(|e| arr.remove(e));
        }
    }

    pub fn can_do(&self, resource: &str, permission: &Privilege) -> bool {
        match self.privileges.get(resource) {
            Some(v) => v.contains(permission),
            None    => false
        }
    }
}

fn with_privilege(resources: &[PathBuf], privileges: &[Privilege]) -> Rights {
    resources.iter().map(|f|
        (f.as_path().to_str().unwrap().to_string(), privileges.to_vec())).collect()
}