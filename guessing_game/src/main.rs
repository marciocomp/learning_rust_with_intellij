use std::io;
use rand::Rng;// used to random number

fn main(){
    println!("Guess the number!");

    let secret_number= rand::thread_rng().gen_range(1, 101); //generate a number. No seed is used - immutable value

    println!("The secret number is: {}",secret_number);

    println!("Please input you guess.");

    let mut guess = String::new();//Storing Values with Variables - mutable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}",guess);
}