pub mod person;
use person::{Person, Gender};

fn main() {
    let soren_name: String = String::from("Soren");
    let soren_age: usize = 20;
    let soren_gender: Gender = Gender::Male;
    let mut soren: Person = Person::new(soren_name, soren_age, soren_gender);
    soren.talk();

    let ajay_name: String = String::from("Ajay");
    let ajay_age: usize = 19;
    let ajay_gender: Gender = Gender::Male;
    let mut ajay: Person = Person::new(ajay_name, ajay_age, ajay_gender);
    ajay.talk();
}