use std::sync::Mutex;
use std::sync::Arc;

#[derive(Debug)]
struct Data (String, String);

impl Data {
    fn new() -> Self {
        Self("Hello".to_string(), "World".to_string())
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    let shared = Arc::new(Mutex::new(Data::new()));
    std::thread::scope(|scope| {
        for i in 0..10 {
            let my_shared = shared.clone();
            scope.spawn(move || {
                let mut lock = my_shared.lock().unwrap();
                lock.0 = format!("{}{i}", lock.0);
            });
        }
    });
    let lock = shared.lock().unwrap();
    println!("{:?}", lock);
}