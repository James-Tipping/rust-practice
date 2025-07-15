use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let mut guess = String::new();
    let secret_number: i32 = rand::thread_rng().gen_range(0..10);

    loop {
        guess.clear(); // Clear the previous guess before reading new input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if "quit" == guess.trim() {
            panic!("You quit the game!");
        }

        println!("You guessed {guess}");
        let num_guess: i32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match num_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Well done!");
                break;
            }
        }
    }

    // if num_guess == secret_number {
    //     println!("You guessed the correct number!");
    // } else {
    //     println!("Wrong! The answer is {secret_number}");
    // }
}
