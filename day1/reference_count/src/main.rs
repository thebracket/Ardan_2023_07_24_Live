use std::rc::Rc;

#[derive(Debug, Clone)]
struct Data(String);

impl Drop for Data {
    fn drop(&mut self) {
        println!("{} was dropped", self.0)
    }
}

fn print(s: Rc<Data>) {
    println!("{s:?}");
}

fn main() {
    let s = Rc::new(Data("hello".to_string()));
    print(s.clone());
    {
        let t = s.clone();
        println!("I have a {t:?}");
    }
}
