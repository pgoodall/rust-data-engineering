/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.
*/

use rand::{rngs::ThreadRng, seq::SliceRandom}; // rand is a random number generation library in Rust
use std::collections::VecDeque;
use cliclack::{intro, outro};
use vecdeque_fruit_salad::actions;

fn main() {
    let _ = match intro("Fruit Salad Generator") {
        Ok(()) => (),
        Err(e) => eprint!("Something went wrong: {}", e),
    };

    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Scramble (shuffle) the fruit
    let mut rng = ThreadRng::default();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the queue after shuffling
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    loop {
        let _ = match actions::choose_action() {
            Ok(a) => { match &a[..] {
                "List" => println!("The fruits currently available are: {:?}\n", fruit),
                "Add" => println!("You can add more fruits to your salad!\n"),
                "Number" => println!("You can change the number of fruits in your salad!\n"),
                "Remove" => println!("You can remove a fruit from your salad!\n"),
                "Exit" => break,
                _ => println!("Invalid choice!\n"),
            }},
            Err(e) => { eprint!("Error choosing action: {}", e)},
        }; 
    }
        
    // Print out the fruit salad
    println!("You final Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    let _ = match outro("I hope you enjoyed your fruit salad!\n") {
        Ok(()) => (),
        Err(e) => eprint!("Something went wrong: {}", e),
    };
}
