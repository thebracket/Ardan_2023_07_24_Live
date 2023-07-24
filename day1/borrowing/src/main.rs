#[derive(Clone)]
struct Data(String);

impl Drop for Data {
    fn drop(&mut self) {
        println!("{} was dropped", self.0);
    }
}

fn print(s: &mut Data) {
    println!("Original: {}", s.0);
    s.0 += "World";
}

fn main() {
    let mut s = Data("Hello".to_string());
    print(&mut s);
    print(&mut s);
    println!("And... we're back");
}