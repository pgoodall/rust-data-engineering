
pub mod actions {
    use cliclack::{select};

    pub fn choose_action() -> Result<String, std::io::Error> {
        let action = select("Use the arrow keys to choose an action below:")
            .item("List", "List the fruits currently available", "")
            .item("Add", "Add frtuit to your salad", "")
            .item("Number", "Change the number of fruits in your salad", "")
            .item("Remove", "Remove a fruit from your salad", "")
            .item("Exit", "Exit", "")
            .interact()?;
        
        Ok(action.to_string())
    }
}