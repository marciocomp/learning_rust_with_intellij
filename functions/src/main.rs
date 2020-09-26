fn main() {
    println!("Hello, world!");
    another_function();
    let a = 10;
    let b = 20;
    let c = new_function(a,b);
    println!("C: {}", c);
}
fn another_function(){
    println!("Another function.");// Testing commit to 1s_branch_functions
    println!("Testing branch!");
    println!("New testing branch!");
}
fn new_function(x: i32, y: i32) -> f64{
    println!("Now I'm using branch to commit on this git project!");
    println!("Testing function");
    let z: f64 = (x as f64) /(y as f64);
    z

}