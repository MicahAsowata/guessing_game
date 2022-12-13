use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    println!("Input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    print!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("It's too small"),
        Ordering::Equal => println!("You win"),
        Ordering::Greater => println!("It's too big"),
    }
}
