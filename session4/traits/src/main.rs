use std::{any::Any, ops::Add};

fn main() {
    let cat = Cat;
    cat.speak();

    let dog = Dog;
    dog.speak();

    animal_speaking(&cat);
    animal_speaking(&dog);

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(cat), Box::new(dog)];
    animals.iter().for_each(|a| a.speak());

    let more_animals: Vec<Box<dyn DowncastableAnimal>> = vec![Box::new(Tortoise)];

    for animal in more_animals.iter() {
        if let Some(t) = animal.as_any().downcast_ref::<Tortoise>() {
            println!("I am a tortoise!")
        }
    }

    println!("Adding Points {:+<50}", "");

    let p1 = Point::new(2.4, 1.4);
    let p2 = Point::new(3.0, 2.5);
    let res = p1 + p2;
    println!("Result is: {:?}", res);
}

trait Animal {
    fn speak(&self);
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow")
    }
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof")
    }
}

fn animal_speaking(a: &impl Animal) {
    a.speak()
}

struct Tortoise;

trait DowncastableAnimal {
    fn speak(&self) {
        println!("No Idea!")
    }
    fn as_any(&self) -> &dyn Any;
}

impl DowncastableAnimal for Tortoise {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
