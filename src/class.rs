pub struct person {
    name: String,
    age: usize,
}

impl person {
    pub fn new(&mut self, age: usize, name: String) {
        person(age, name)
    }
}