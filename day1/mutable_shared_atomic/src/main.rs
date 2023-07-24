use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;

static SHARED: AtomicI32 = AtomicI32::new(0);

fn main() {
    std::thread::scope(|scope| {
        for _ in 0..1000 {
            scope.spawn(|| {
                for _ in 0 .. 1000 {
                    SHARED.fetch_add(1, Ordering::Relaxed);
                }
            });
        }
    });
    println!("{}", SHARED.load(Ordering::Relaxed));
}