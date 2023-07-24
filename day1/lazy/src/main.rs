use once_cell::sync::Lazy;

static SHARED: Lazy<String> = Lazy::new(|| "Hello".to_string());

fn main() {
    println!("{}", *SHARED);
}