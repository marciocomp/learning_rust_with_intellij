fn main() {

    let mut guess: f32 = "42.09".parse().expect("Not a number!");// We must use type annotation e.g: f32
    guess+=1.567;//just testing sum
    println!("Guess: {}",guess);

}
