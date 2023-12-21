use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    // Generate a secret number between 1 and 100.
    let secret_num = rand::thread_rng().gen_range(1..101);
    loop {
        println!("------------------------");
        println!("Please input your guess.");
        // Read from the stardard input.
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Try to parse the input to a number.
        // Continue to the next iteration immediately when the parse failed.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_num) {
            Ordering::Equal => {
                println!("You won!");
                break;
            }
            Ordering::Greater => println!("Too large."),
            Ordering::Less => println!("Too small."),
        }
        println!("------------------------");
    }
}
