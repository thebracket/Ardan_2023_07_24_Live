use tokio::process::Command;

#[tokio::main]
async fn main() {
    let result = Command::new("../../target/release/thumbnailer")
        .args(["../photo.jpg", "thumbnail.jpg"])
        .output()        
        .await;

    if let Ok(output) = result {
        let returned_text = String::from_utf8(output.stdout).unwrap();
        println!("Process returned: {returned_text}");
    }
}