struct Data(i32);

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Data").field(&self.0).finish()
    }
}

fn main() {
    let data = Data(12);
    println!("{data:?}");
}
