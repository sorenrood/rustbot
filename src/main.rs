mod person;
use person::Person;

fn main() {
    println!("We are going to construct a cool struct now");
    let name = String::from("Soren");
    let age: usize = 20;
    let mut soren = Person::new(name, age);
    soren.talk();
}