#[derive(Debug, Clone)]
enum UsersError {
    NoUsers, TooManyUsers
}

use std::fmt;

impl fmt::Display for UsersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UsersError::NoUsers => write!(f, "no users found"),
            UsersError::TooManyUsers => write!(f, "too many users found"),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
