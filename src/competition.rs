use crate::person::Person;
pub struct Competition {
    pub players: Option<Vec<Person>>,
}

impl Competition {
    pub fn new() -> Competition {
        Competition { players: None }
    }

    pub fn run() {
        // Start the competition
    }
}