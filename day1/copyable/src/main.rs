fn print(n: i32) {
    println!("{n}");
}


#[derive(Copy, Clone)]
struct Data(i32);

fn main() {
    let n = 12;
    print(n);
    println!("{n}");
}
