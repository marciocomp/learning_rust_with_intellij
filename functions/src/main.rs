fn main() {
    println!("Hello, world!");
    another_function();
    let a = 10;
    let b = 20;
    let c = new_function(a,b);
    println!("C: {}", c);
    let d = {
        let a = 50;
        a + 1
    };
    let a= five();
    println!("A: {}",a);
    println!("D: {}",d);
    let a = plus_one(five());
    let mut b = plus_one(a);
    b=plus_one(b);
    println!("A: {}",a);
    println!("B: {}",b);
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
fn five()-> i32{
    5
}
fn plus_one(x:i32) -> i32{
    x + 1
}