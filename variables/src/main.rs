fn main() {
    //Testing CONSTANTS
    const MAX_POINTS: u32 = 100_000;// The underscore helps us to understand 100,000. We can use,
                                    // for instance, 1_000_000 that means 1,000,000. The underscore
                                    // does not influence in the final value of the constant.
    let x = 5;//default type for integer: i32
    println!("The value of x is: {}", x);

    //Testing shadow variable
    let x = MAX_POINTS;//trying constant MAX_POINTS and shadowing. The  new type of x is u32
                            // because MAX_POINTS is u32
    println!("The value of x is: {}", x);

    let x = x + 1; // 2nd shadow. Type of x is still u32 because last x was u32
    let x = x * 2; // 3rd shadow

    println!("The value of x is: {}", x);




}