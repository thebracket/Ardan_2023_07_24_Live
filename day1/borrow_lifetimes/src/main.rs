#[derive(Clone, Debug)]
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

fn get_ref<'a>(s: &'a Data, t: &'a Data) -> &'a Data {
    t
}

struct Borrowed<'a> {
    data: &'a Data,
}

fn main() {
    let s = Data("Hello".to_string());
    let t = get_ref(&s, &s);
    println!("{t:?}");
    std::mem::drop(s);
    println!("{t:?}");
}