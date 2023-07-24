use std::sync::RwLock;

static SHARED: RwLock<String> = RwLock::new(String::new());

fn main() {
    std::thread::scope(|scope| {
        for _ in 0..1000 {
            scope.spawn(|| {
                for _ in 0 .. 1000 {
                    let mut lock = SHARED.write().unwrap();
                    *lock += "1";
                }
            });
        }
    });

    let read_lock = SHARED.read().unwrap();
    println!("{read_lock:?}");
}