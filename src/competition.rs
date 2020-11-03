use crate::person::Person;

/// This is a struct to model a competition.
pub struct Competition {
    pub people: Option<Vec<Person>>,
}

impl Competition {
    /// Will return a compettion struct.
    pub fn new() -> Competition {
        Competition { people: None }
    }

    /// A function that will run the competition. 
    pub fn run(&mut self) {
        println!("Starting the competition.");
    }

    pub fn add_people(&mut self, list_of_people: Vec<Person>) {
        self.people = Some(list_of_people);
    }

    pub fn display_people(&mut self) {
        match &self.people {
            Some(people) => {
                for person in people {
                    println!("{}", person.name);
                }
            },
            None => {
                println!("No people found");
            },
        }
    }
}