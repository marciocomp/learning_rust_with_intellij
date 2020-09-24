use std::io;
use rand::Rng;// used to random number
use std::cmp::Ordering;
use std::io::BufRead;
//use std::fs::read;

fn main(){
    println!("Guess the number!");

    let secret_number= rand::thread_rng().gen_range(1, 101); //generate a number. No seed is used - immutable value
    // println!("The secret number is: {}",secret_number);
    let mut count= 0;
    loop {
        count+=1;
        //    println!("Please input you guess: ");
        println!("Please type the {}[st,nd,rd,th] magic number: ",count);

        let magic: i32 = io::stdin().lock()//trying to input integer
            .lines()
            .next()
            .expect("stdin should be available")
            .expect("couldn't read from stdin")
            .trim()
            .parse()
            .expect("input was not an integer");
        println!("The magic number is: {}",magic);


        //    let mut guess = String::new();//Storing Values with Variables - mutable

        //  io::stdin()
        //       .read_line(&mut guess)
        // .expect("Failed to read line");

        // let guess:u32 = match guess.trim()
        //     .parse() {
        //     Ok(num) => num,               //verifying invalid input
        //     Err(_) => {
        //       println!("Please type a number!");
        //       continue;
        //     }

        // };//.expect("Please type a number!");//It converts string to number -> u32

        // println!("You guessed: {}",guess);
        match magic.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}