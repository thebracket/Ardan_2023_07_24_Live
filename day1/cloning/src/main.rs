#[derive(Clone)]
struct Data(String);

impl Drop for Data {
    fn drop(&mut self) {
        println!("{} was dropped", self.0);
    }
}

fn print(s: Data) {
    println!("Hello {}", s.0);
}

fn main() {
    let s = Data("Hello".to_string());
    print(s.clone());
    print(s);
    println!("And... we're back");
}