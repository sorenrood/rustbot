fn main() {
    println!("Hello, world!");
}

pub struct Person {
    name: String,
    age: usize,
}

impl Person {
    pub fn new(&mut self, age: usize, name: String) {
        Person { name: name, age: age };
    }
}