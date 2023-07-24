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

    let lock = SHARED.lock().unwrap();
    std::mem::drop(lock);
    let lock2 = SHARED.lock().unwrap();


    // do stuff
    {
        let lock = SHARED.lock().unwrap();
        // do more stuff that needs the lock
    }
    // Do even more stuff
    let lock2 = SHARED.lock().unwrap();
    // One last lock
}
