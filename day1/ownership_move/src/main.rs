fn print(s: String) -> String {
    println!("{s}");
    s
}

fn main() {
    let s= "Hello".to_string();
    let s2 = print(s);
    println!("{s2}");
}
