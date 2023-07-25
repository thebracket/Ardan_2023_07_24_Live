use std::sync::mpsc;

enum Command {
    SayHello, Quit, Say(String),
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();

    let handle = std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::SayHello => println!("Hello"),
                Command::Say(message) => println!("Hello {message}"),
                Command::Quit => {
                    println!("Quitting now");
                    break;
                }
            }
        }
    });

    for i in 0 .. 10 {
        tx.send(Command::SayHello).unwrap();
        tx.send(Command::Say(format!("{i}"))).unwrap();
    }
    println!("Sending Quit");
    tx.send(Command::Quit).unwrap();
    handle.join().unwrap();
}
