use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};

pub fn greet(name: &str) -> String {
    format!("Welcome back {name}!")
}

fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize())
}

pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
    role: LoginRole,
}

impl User {
    fn new(username: &str, password: &str, role: LoginRole) -> Self {
        Self {
            username: username.to_lowercase(),
            password: hash_password(password),
            role,
        }
    }
}

// fn get_users() -> Vec<User> {
//     vec![
//         User::new("admin", "nimda", LoginRole::Admin),
//         User::new("moha", "bamoh", LoginRole::User),
//     ]
// }

fn get_example_users() -> HashMap<String, User> {
    let mut users = HashMap::new();
    users.insert(
        "admin".to_string(),
        User::new("admin", "nimda", LoginRole::Admin),
    );
    users.insert(
        "moha".to_owned(),
        User::new("moha", "bamoh", LoginRole::User),
    );
    users
}

fn get_users() -> HashMap<String, User> {
    let users_path = Path::new("users.json");
    if users_path.exists() {
        let users_json = std::fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();
        users
    } else {
        let users = get_example_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}

// fn get_admin_usernames() -> Vec<String> {
//     get_users()
//         .into_iter() // into_iter moves ownership
//         .filter(|u| u.role == LoginRole::Admin)
//         .map(|u| u.username)
//         .collect()
// }

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username_lower = username.to_lowercase();
    let password = hash_password(password);
    // lookup by hashkey is faster than iterating over vector
    if let Some(user) = get_users().get(&username_lower) {
        // if let Some(user) = get_users().iter().find(|u| u.username == username_lower) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }
    None

    // if username.to_lowercase() != "admin" && username.to_lowercase() != "moha" {
    //     return None;
    // }

    // if username.to_lowercase() == "admin" && password == "nimda" {
    //     Some(LoginAction::Granted(LoginRole::Admin))
    // } else if username.to_lowercase() == "moha" && password == "ahom" {
    //     Some(LoginAction::Granted(LoginRole::User))
    // } else {
    //     Some(LoginAction::Denied)
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!("Welcome back Mohamed!", greet("Mohamed"))
    }

    #[test]
    fn test_login() {
        // assert!(login("admin", "nimda"));
        // assert!(login("AdMIn", "nimda"));
        // assert!(!login("username", "nimda"));
        // assert!(!login("admin", "password"))
    }
}
