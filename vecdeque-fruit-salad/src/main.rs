/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.
*/

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
// rand is a random number generation library in Rust
use std::collections::VecDeque;
use cliclack::{intro, outro};
use vecdeque_fruit_salad::actions;

fn main() {
    let _ = match intro("Fruit Salad Generator") {
        Ok(()) => (),
        Err(e) => eprint!("Something went wrong: {}", e),
    };

    //let mut fruit_iter: VecDeque<String> = VecDeque::new();
    let mut fruit: VecDeque<String> = VecDeque::new();
    fruit.push_back("Arbutus".to_string());
    fruit.push_back("Loquat".to_string());
    fruit.push_back("Strawberry Tree Berry".to_string());

    // Scramble (shuffle) the fruit
    let mut rng = ThreadRng::default();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the queue after shuffling
    fruit.push_front("Pomegranate".to_string());
    fruit.push_back("Fig".to_string());
    fruit.push_back("Cherry".to_string());

    loop {
        let _ = match actions::choose_action() {
            Ok(a) => { match &a[..] {
                "List" => println!("The fruits currently available are: {:?}\n", fruit),
                "Add" => { let new_fruits = match actions::add() {
                                Ok(s) => s,
                                Err(e) => panic!("{e}"),                         
                            };

                            let mut fruit_iter: VecDeque<String> = new_fruits.split("\n").map(str::to_string).collect();
                            fruit_iter.pop_back_if(|x| x == "");
                            fruit.append(&mut fruit_iter);
                        },
                "Random" => actions::random(&fruit),
                "Remove" => { fruit = actions::remove(fruit).expect("Error returning modified list.") },
                "Exit" => { break;
                    },
                _ => eprint!("This should never happen!"),

                }
            },
            Err(e) => eprint!("{}", e),
        };
    }
                        
    //let mut fruit_iter: VecDeque<String> = new_fruits.split_whitespace().map(str::to_string).collect();
    // for line in fruit_iter {
    //     vec2.push_back(&line);
    // }
    //fruit.append(&mut fruit_iter);

    // Print out the fruit salad
    println!("Your final Fruit Salad:");
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
