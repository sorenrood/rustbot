#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
    Other,
}

pub struct Person {
    name: String,
    age: usize,
    gender: Gender,
}

impl Person {
    pub fn new(n: String, a: usize, g: Gender) -> Person {
        Person {
            name: n, 
            age: a,
            gender: g,
        }
    }

    pub fn talk(&mut self) {
        println!("Hi I'm {}! I am {} years old. I identify as a {:?}.", self.name, self.age, self.gender);
    }
}