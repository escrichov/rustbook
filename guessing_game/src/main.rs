// Here’s how it works: the program will generate a random integer between 1 and 100.
// It will then prompt the player to enter a guess.
// After a guess is entered, the program will indicate whether the guess is too low or too high.
// If the guess is correct, the game will print a congratulatory message and exit.
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess game!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
         println!("Please enter your guess:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
