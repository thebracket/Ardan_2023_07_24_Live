use std::sync::Mutex;
use std::sync::Arc;

#[derive(Debug)]
struct Data (Mutex<String>, String);

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
    let shared = Arc::new(Data::new());
    std::thread::scope(|scope| {
        for i in 0..10 {
            let my_shared = shared.clone();
            scope.spawn(move || {
                let mut lock = my_shared.0.lock().unwrap();
                *lock = format!("{}{i}", lock);
            });
        }
    });
    let lock = shared.0.lock().unwrap();
    println!("{}, {}", *lock, shared.1);
}