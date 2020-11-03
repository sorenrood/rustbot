use crate::person::Person;

/// This is a struct to model a competition.
pub struct Competition {
    pub players: Option<Vec<Person>>,
}

impl Competition {
    pub fn new() -> Competition {
        Competition { players: None }
    }

    /// A function that will run the competition. 
    pub fn run() {
        // Start the competition
    }
}