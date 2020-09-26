pub fn run(){
    // loop{
    //     println!("Again!"); //it executes forever :0
    // }
    println!("The result is: {}", first(20));
  }
fn first(breaking:i32) -> i32{
    let mut count = 0;

    let result = loop{
        count +=1;
        if count == breaking{
            break count * 2
        }
    };
    result
}