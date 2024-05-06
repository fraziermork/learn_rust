use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("You are trying to guess: {secret_number}");

    loop {
        println!("Input your guess:");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed");
    
        println!("You guessed: {guess}");
    
        let parsed_guess: u32 = match guess.trim().parse().expect("Please input a number") {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match parsed_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            },
        }

    }


}
