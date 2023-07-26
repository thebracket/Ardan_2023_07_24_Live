use std::path::Path;
use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize)]
struct User {
    name: String,
    password: String,
}

#[derive(Debug, Error)]
enum UsersError {
    #[error("No users were found")]
    NoUsers,
    #[error("Too many users were found")]
    TooManyUsers,
    #[error("Unable to open users database")]
    ReadError,
    #[error("Unable to deserialize json")]
    JsonError(serde_json::Error),
}

fn work_with_my_error() -> Result<Vec<User>, UsersError> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)
        .map_err(|_| UsersError::ReadError)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)
        .map_err(|e| UsersError::JsonError(e))?;
    if users.is_empty() {
        Err(UsersError::NoUsers)
    } else if users.len() > 10 {
        Err(UsersError::TooManyUsers)
    } else {
        Ok(users)
    }
}

fn main() {
    println!("Hello, world!");
}
