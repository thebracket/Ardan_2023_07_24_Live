use std::path::Path;
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    name: String,
    password: String,
}

fn anyhow_load_users() -> anyhow::Result<Vec<User>> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    Ok(users)
}

#[allow(dead_code)]
fn anyhow_load_users2() -> anyhow::Result<Vec<User>> {
    let my_file = Path::new("users.json");
    let raw_text = std::fs::read_to_string(my_file)?;
    let users: Vec<User> = serde_json::from_str(&raw_text)?;
    if users.is_empty() {
        anyhow::bail!("No users found");
    }
    if users.len() > 10 {
        return Err(anyhow::Error::msg("Too many users"));
    }
    Ok(users)
}

fn main() {
    let users = anyhow_load_users();
    match users {
        Ok(users) => {
            for user in users {
                println!("User: {}, {}", user.name, user.password);
            }
        },
        Err(err) => {
            println!("Error: {err}");
        }
    }
}
