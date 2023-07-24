trait Animal {
    fn speak(&self);
    fn make_noise(&self) {
        println!("???");
    }
}

trait DebuggableAnimal: std::fmt::Debug + Clone {
    fn speak(&self);
}

#[derive(Debug)]
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

fn speak_twice(animal: &impl Animal) {
    animal.speak();
    animal.speak();
}

fn get_animal() -> impl Animal {
    Cat
}

fn main() {
    let cat = Cat;
    cat.speak();
    let dog = Dog;
    dog.speak();
    speak_twice(&cat);
    let my_animal = get_animal();

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)];
    for animal in animals.iter() {
        animal.speak();
    }
}
