fn main() {

    println!("Condition was {}", first());
    println!("Condition was {}", second());
    let a = 10;
    let b =third(a);
    if b !="0" {
        println!("The number {} is divisible by {}",a,b);
    }else {
        println!("The number {} is not divisible by 4, 3, or 2", a);
    }
    println!("{}",third(a));
    println!("The value of number in fourth function is: {}", fourth());

}
fn first() -> bool{ //1st example
    let number = 3;
    if number < 5{
        true
    }else{
        false
    }
}
fn second() -> bool{
    let number = 7;
    if number < 5 { true } else { false }
}
fn third(number: i32) -> String{
    if number % 4 == 0 {
        4.to_string()
    }else if number % 3 == 0 {
        3.to_string()
    }else if number % 2 == 0 {
        2.to_string()
    }else {
        0.to_string()
    }
}
fn fourth() -> i32{
    let condition = true;
    let number = if condition { 5 } else { 6 };
    number
}