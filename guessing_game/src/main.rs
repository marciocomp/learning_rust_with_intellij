use std::io;
use rand::Rng;// used to random number
use std::cmp::Ordering;

fn main(){
    println!("Guess the number!");

    let secret_number= rand::thread_rng().gen_range(1, 101); //generate a number. No seed is used - immutable value

    println!("The secret number is: {}",secret_number);

    println!("Please input you guess.");

    let mut guess = String::new();//Storing Values with Variables - mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess:u32 = guess.trim()
        .parse().expect("Please type a number!");//It converts string to number -> u32

    println!("You guessed: {}",guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}