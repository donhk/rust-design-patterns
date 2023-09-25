use std::collections::HashMap;

struct FlyWeight<'a> {
    shared_state: &'a str,
}

impl<'a> FlyWeight<'a> {
    fn new(shared_state: &'a str) -> Self {
        FlyWeight { shared_state }
    }
    fn set_state(&mut self, shared_state: &'a str) {
        self.shared_state = shared_state;
    }

    fn get_state(&self) -> &str {
        self.shared_state
    }
}

struct FlyWeightFactory<'a> {
    flyweights: HashMap<&'a str, FlyWeight<'a>>,
}

impl<'a> FlyWeightFactory<'a> {
    fn new() -> Self {
        FlyWeightFactory {
            flyweights: HashMap::new(),
        }
    }

    fn get_flyweight(&mut self, key: &'a str) -> &mut FlyWeight<'a> {
        if !self.flyweights.contains_key(key) {
            self.flyweights.insert(key, FlyWeight::new(key));
        }
        self.flyweights.get_mut(key).unwrap()
    }
}

fn main() {
    let mut factory = FlyWeightFactory::new();
    let flyweight_a = factory.get_flyweight("hello");
    flyweight_a.set_state("hello world");
    println!("{}", flyweight_a.get_state());

    let flyweight_b = factory.get_flyweight("world");
    flyweight_b.set_state("world hello");
    println!("{}", flyweight_b.get_state());

    let flyweight_c = factory.get_flyweight("hello");
    println!("{}", flyweight_c.get_state());
}
