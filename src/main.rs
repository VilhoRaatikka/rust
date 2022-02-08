use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..100);
    loop {
        println!("Guess the number\n");
        println!("Input your guess : ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        if guess.trim().to_string() == "quit".trim().to_string() {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("WTF? Again.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Match");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}
