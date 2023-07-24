use std::sync::Mutex;

static SHARED: Mutex<String> = Mutex::new(String::new());

fn main() {
    std::thread::scope(|scope| {
        for _ in 0..1000 {
            scope.spawn(|| {
                for _ in 0 .. 1000 {
                    let mut lock = SHARED.lock().unwrap();
                    *lock += "1";
                }
            });
        }
    });
    let lock = SHARED.lock().unwrap();
    println!("{}", *lock);
}
