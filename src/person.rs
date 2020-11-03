pub struct Person {
    name: String,
    age: usize,
}

impl Person {
    pub fn new(n: String, a: usize) -> Person {
        Person {
            name: n, 
            age: a 
        }
    }

    pub fn talk(&mut self) {
        println!("Hi, my name is {} and I am {} years old.", self.name, self.age);
    }
}