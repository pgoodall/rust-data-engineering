
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

    pub fn remove(fruit: VecDeque<String>) -> Result<VecDeque<String>, &'static str> {
        // convert fruit to Vec for autocomplete        
        let fruit_list: Vec<String> = fruit.clone().into_iter().collect();

        // Take user input with autocomplete
        let remove: String = match Input::new("Choose a fruit to remove") 
            .autocomplete(fruit_list)
            .interact() {
                Ok(s) => s,
                Err(e) => return Err("Error entering fruit."),
            };

        // convert back to VecDeque 
        let mut fruit: VecDeque<String> = fruit.into_iter().collect();

        // Check if the input matches the list of fruit
        let mut index = None;
        for (i, item) in fruit.iter().enumerate() {
            if item == &remove {
                index = Some(i);
            } 
        }

        // If no match is made, return an error
        let fruit_index = match index {
            Some(i) => i,
            None => return Err("Invalid input for fruit.")
        };

        println!("You removed {} from the list.\n", fruit.remove(fruit_index).expect("Error finding fruit."));

        Ok(fruit)
    }
}