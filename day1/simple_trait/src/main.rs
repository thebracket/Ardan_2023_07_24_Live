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

struct Tortoise {
    shell_size: i32,
}

impl Animal for Tortoise {
    fn speak(&self) {
        println!("What noise does a tortoise make anyway?");
    }
}

use std::any::Any;

trait DowncastableAnimal {
    fn as_any(&self) -> &dyn Any;
}

impl DowncastableAnimal for Tortoise {
    fn as_any(&self) -> &dyn Any {
        self
    }
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

    let more_animals : Vec<Box<dyn DowncastableAnimal>> = vec![Box::new(Tortoise)];
    for animal in more_animals.iter() {
        //animal.shell_size <-- Not available
        if let Some(tortoise) = animal.as_any().downcast_ref::<Tortoise>() {
            println!("It's a tortoise");
            tortoise.shell_size;
        }
    }
}
