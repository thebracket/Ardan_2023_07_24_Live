struct Data(String);

impl Drop for Data {
    fn drop(&mut self) {
        println!("{} was dropped", self.0);
    }
}

fn print(s: Data) -> Data {
    println!("Hello {}", s.0);
    return s;
}

fn main() {
    {
        let t = Data("Test".to_string());
    }
    let s = Data("Hello".to_string());
    let s = print(s);
    println!("And... we're back");
}