static mut SHARED: i32 = 0;

fn main() {
    std::thread::scope(|scope| {
        for _ in 0..1000 {
            scope.spawn(|| unsafe {
                for _ in 0 .. 1000 {
                    SHARED += 1;
                }
            });
        }
    });
    unsafe {
        println!("{SHARED}");
    }
}
