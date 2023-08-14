use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please guess the number!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win!");
                println!(
                    "You guessed: {}, the secret number was: {}",
                    guess, secret_number
                );
                break;
            }
            Ordering::Greater => {
                println!("Too High!");
                continue;
            }
            Ordering::Less => {
                println!("Too Low!");
                continue;
            }
        }
    }
}
