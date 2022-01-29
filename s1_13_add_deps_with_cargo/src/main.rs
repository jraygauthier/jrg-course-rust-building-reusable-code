use rand::random;
use std::io;

fn get_guest() -> u8 {
    loop {
        println!("Input guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read from stdin");

        match guess.trim().parse::<u8>() {
            Ok(v) => return v,
            Err(e) => println!("Could not understand input: {}", e),
        }
    }

    // If the parse fails, this version would exit the program with the
    // provided message.
    // return guess.parse::<u8>().expect("You didn't enter an int!");
}

fn handle_guest(guess: u8, correct: u8) -> bool {
    if guess < correct {
        println!("Too low.");
        false
    } else if guess > correct {
        println!("Too high.");
        false
    } else {
        println!("You got it!");
        true
    }
}

fn main() {
    println!("Welcome to the guessing game!");
    let correct: u8 = random::<u8>();
    println!("Correct value is: {}", correct);

    loop {
        let guess = get_guest();
        if handle_guest(guess, correct) {
            break;
        }
    }
}
