pub fn run(){
    // loop{
    //     println!("Again!"); //it executes forever :0
    // }
    println!("The result is: {}", first(20));
    second();
    third();
    println!("***********");
    fourth();
    println!("***********");
    fifth();
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
fn second(){
    let mut number = 3;

    while number != 0{
        println!("{}!",number);
        number -=1;
    }
    println!("LIFTOFF");
}
fn third(){
    let a = [10,20,30,40,50];
    let mut i = 0;

    while i < 5{
        println!("The value a[{}] is: {}",i,a[i]);
        i +=1;
    }
}
fn fourth(){
    let a = [10,20,30,40,50];
    let mut i = 0;
    for element in a.iter(){
        println!("The value a[{}]is: {}",i, element);
        i +=1;
    }
}
fn fifth(){
    let a = [10,20,30,40,50];

    for number in (0..5).rev(){
        println!("The value a[{}] is: {}", number,a[number]);
    }
}


