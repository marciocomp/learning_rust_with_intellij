fn main() {

    let x = 5;
    println!("The value of x is: {}", x);

    x = 6;//error because x is an immutable variable
    println!("The value of x is: {}", x);
}