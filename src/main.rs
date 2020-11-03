mod person;
use person::{Person, Gender};

fn main() {
    let name = String::from("Soren");
    let age: usize = 20;
    let gender = Gender::Male;
    let mut soren = Person::new(name, age, gender);

    soren.talk();
}