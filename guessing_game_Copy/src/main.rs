use std::cmp::Ordering;
use std::{io};
use fastrand;
fn main() {
    println!("Guess the number!");
    let a='a';
    let z='z';
    let secret_letter = fastrand::char(a..=z);
    println!("The secret letter is: {}",secret_letter);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: char = match guess.trim().parse() {
            Ok(char)=> char,
            Err(_) => continue,
        };
        if guess>=a && guess<=z {
            println!("You guessed: {}", guess);

            match guess.cmp(&secret_letter) {
                Ordering::Less => println!("Letter is too small!"),
                Ordering::Greater => println!("Letter is too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
        else {
            continue;
        }
    }    
}
