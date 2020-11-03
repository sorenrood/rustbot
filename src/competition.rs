use crate::person::Person;

/// This is a struct to model a competition.
pub struct Competition {
    pub players: Option<Vec<Person>>,
}

impl Competition {
    /// Will return a compettion struct.
    pub fn new() -> Competition {
        Competition { players: None }
    }

    /// A function that will run the competition. 
    pub fn run(&mut self) {
        // Start the competition
    }
}