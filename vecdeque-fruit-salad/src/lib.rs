
pub mod actions {
    use std::collections::VecDeque;
    use cliclack::{Input, select};
    use rand::rng;

    pub fn choose_action() -> Result<String, std::io::Error> {
        let action = select("Use the arrow keys to choose an action below:")
            .item("List", "List the fruits currently available", "")
            .item("Add", "Add frtuit to your salad", "")
            .item("Random", "Choose a random fruit from the list", "")
            .item("Remove", "Remove a fruit from your salad", "")
            .item("Exit", "Exit", "")
            .interact()?;
        
        Ok(action.to_string())
    }

    pub fn add() -> Result<String, std::io::Error> {
        let new_fruits = Input::new("Add one fruit per line:")
            .multiline()
            .interact()?;

        Ok(new_fruits)
    }

    pub fn random(fruit: &VecDeque<String>) {
        use rand::seq::IteratorRandom;

        println!("Random fruit: {}\n", fruit.iter().choose(&mut rng()).unwrap());

    }
}