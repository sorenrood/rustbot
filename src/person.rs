/// An enum that represents a very simple gender struct.
#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Other,
}

/// A struct that represents a person.
pub struct Person {
    name: String,
    age: usize,
    gender: Gender,
}

impl Person {
    /// Will return a person
    pub fn new(n: String, a: usize, g: Gender) -> Person {
        Person {
            name: n, 
            age: a,
            gender: g,
        }
    }

    /// Will print the name, age, and gender of a person
    pub fn talk(&mut self) {
        println!("Hi I'm {}! I am {} years old. I identify as a {:?}.", self.name, self.age, self.gender);
    }
}