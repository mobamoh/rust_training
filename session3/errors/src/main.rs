use std::path::Path;

use serde::Deserialize;

fn main() {
    let path = Path::new("my_file.txt");
    let res = std::fs::read_to_string(path);
    // println!("{res:#?}");
    match res {
        Ok(content) => println!("Content: {content}"),
        // Err(e) => println!("Error dealing with file"),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                println!("File {:?} not found", path.file_name().unwrap())
            }
            _ => println!("{e}"),
        },
    }
}

fn read_file() -> Result<String, std::io::Error> {
    let path = Path::new("my_file.txt");
    std::fs::read_to_string(path)
}

fn read_file_to_uppercase() -> Result<String, std::io::Error> {
    let path = Path::new("my_file.txt");
    let content = std::fs::read_to_string(path)?; // in case of error this propagates back to the caller
    Ok(content.to_uppercase())
}

#[derive(Deserialize, Debug)]
struct User {
    name: String,
}

fn load_users() -> Result<Vec<User>, std::io::Error> { // Error mismatch???
    let path = Path::new("users.json");
    let content = std::fs::read_to_string(path)?;
    let users: Vec<User> = serde_json::from_str(&content)?;
    Ok(users)
}

type GenericErrorResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// more of a client side programming, not appropriate for libraries
fn load_users_dyn_error() -> GenericErrorResult<Vec<User>> {
    //dynamic at runtime
    let path = Path::new("users.json");
    let content = std::fs::read_to_string(path)?;
    let users: Vec<User> = serde_json::from_str(&content)?;
    Ok(users)
}

fn load_users_anyhow() -> anyhow::Result<Vec<User>> {
    //dynamic at runtime
    let path = Path::new("users.json");
    let content = std::fs::read_to_string(path)?;
    let users: Vec<User> = serde_json::from_str(&content)?;
    // anyhow::bail!("something went wrong!"); // returns an Error
    Ok(users)
}

// For libraries better use thiserror
// Return custom Errors
#[derive(Debug, thiserror::Error)]
enum UserError {
    #[error("no user found")]
    NoUsers,
    #[error("too many users found")]
    TooManyUsers,
}

fn load_users_thiserror() -> Result<Vec<User>, UserError> {
    //dynamic at runtime
    let path = Path::new("users.json");
    let content = std::fs::read_to_string(path).map_err(|_| UserError::NoUsers)?;
    let users: Vec<User> = serde_json::from_str(&content).map_err(|_| UserError::TooManyUsers)?;
    // anyhow::bail!("something went wrong!"); // returns an Error
    Ok(users)
}


// Same for async -> Future -> .await?