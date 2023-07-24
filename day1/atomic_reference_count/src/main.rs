use std::sync::Arc;

#[derive(Debug, Clone)]
struct Data(String);

impl Drop for Data {
    fn drop(&mut self) {
        println!("{} was dropped", self.0)
    }
}

fn main() {
    let s = Arc::new(Data("hello".to_string()));

    std::thread::scope(|scope| {
        for _ in 0..10 {
            let my_s = s.clone();
            scope.spawn(move || println!("{my_s:?}"));
        }
    });
}
