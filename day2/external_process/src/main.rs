use std::process::Command;

fn main() {
    let result = Command::new("../../target/release/thumbnailer")
        .args(["../photo.jpg", "thumbnail.jpg"])
        .spawn();
    println!("{result:?}");
}