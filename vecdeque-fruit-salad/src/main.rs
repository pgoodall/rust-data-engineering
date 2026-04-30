/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::VecDeque;
use cliclack::{intro, outro, select};

fn main() {
    intro("Fruit Salad Generator");

    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the queue after shuffling
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    let action = select("What would you like to do? (choose from the list below)")
        .item("List", "List the fruits currently available", "")
        .item("Add", "Add frtuit to your salad", "")
        .item("Number", "Change the number of fruits in your salad", "")
        .item("Remove", "Remove a fruit from your salad", "")
        .interact();

    match action {
        Ok(c) => match c {
            "List" => println!("The fruits currently available are: {:?}", fruit),
            "Add" => println!("You can add more fruits to your salad!"),
            "Number" => println!("You can change the number of fruits in your salad!"),
            "Remove" => println!("You can remove a fruit from your salad!"),
            _ => println!("Invalid choice!"),
        },
        Err(e) => eprint!("{}", e),

    };
        
    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

     outro("I hope you enjoyed your fruit salad!");
}
