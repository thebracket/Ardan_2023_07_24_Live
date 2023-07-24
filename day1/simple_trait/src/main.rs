trait Animal {
    fn speak(&self);
}

struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}

struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof");
    }
}

fn main() {
    let cat = Cat;
    cat.speak();
    let dog = Dog;
    dog.speak();
}
