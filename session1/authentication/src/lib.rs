pub fn greet(name: &str) -> String {
    format!("Welcome back {name}!")
}

pub fn login(username: &str, password: &str) -> bool {
    username.to_lowercase() == "admin" && password == "nimda"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet(){
        assert_eq!("Welcome back Mohamed!",greet("Mohamed"))
    }

    #[test]
    fn test_login(){
        assert!(login("admin", "nimda"));
        assert!(login("AdMIn", "nimda"));
        assert!(!login("username", "nimda"));
        assert!(!login("admin", "password"))
    }
}
