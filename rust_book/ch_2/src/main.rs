use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_num: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Greater => println!("Number {guess} is too big"),
            Ordering::Less => println!("Number {guess} is too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
