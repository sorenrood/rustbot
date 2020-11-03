pub mod person;
pub mod competition;
use person::{Person, Gender};
use competition::Competition;

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

    let steven_name: String = String::from("Steven");
    let steven_age: usize = 19;
    let steven_gender: Gender = Gender::Male;
    let mut ajay: Person = Person::new(steven_name, steven_age, steven_gender);
    ajay.talk();

    let karsten_name: String = String::from("Steven");
    let karsten_age: usize = 19;
    let karsten_gender: Gender = Gender::Male;
    let mut karsten: Person = Person::new(karsten_name, karsten_age, karsten_gender);
    karsten.talk();

    let mut competition = Competition::new();
    competition.run();
}