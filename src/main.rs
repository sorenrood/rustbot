pub struct Person {
    name: String,
}

impl Person {
}

fn main() {
    let name = String::from("Soren");
    let soren = Person { name: name };
    println!("Person's Name: {}", soren.name);
}