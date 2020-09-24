fn main() {
    //Testing CONSTANTS
    const MAX_POINTS: u32 = 100_000;// The underscore helps us to understand 100,000. We can use,
                                    // for instance, 1_000_000 that means 1,000,000. The underscore
                                    // does not influence in the final value of the constant.
    let mut x = 5;//default type for integer: i32
    println!("The value of x is: {}", x);

    x = MAX_POINTS;//trying constant MAX_POINTS
    println!("The value of x is: {}", x);
}