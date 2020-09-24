fn main() {

    let mut guess: f32 = "42.09".parse().expect("Not a number!");// We must use type annotation e.g: f32
    guess+=1.567;//just testing sum
    println!("Guess: {}",guess);

    //scalar type -> single value -> four primary scalar types: integer, floating-point numbers,
    //booleans, and characters

    /************INTEGER TYPES************/
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte: u8 = b'A';//only use u8 for "Byte" subtype

    println!("Decimal: {}",decimal);// we can use annotation format to print as you want "to see"
    println!("hex: {:0x}",hex);// We can use any format, for instance: Variable hex -> {:0b} -> It'll
                                // print in binary form
    println!("octal: {:0o}",octal);
    println!("binary: {:0b}",binary);
    println!("byte: {:b}",byte);

    /************END INTEGER TYPES************/

    /************FLOATING-POINT TYPES************/

        let mut float_64 = 2.0; //the default is f64 -> Double precision
        let mut float_32: f32 = 3.0; //single precision
        float_64 += float_32;
        float_32 = float_64+1.0;
        println!("Sum of float 64: {}",float_64);
        println!("Sum of float 32: {}",float_32);

    /************END FLOATING-POINT TYPES************/


}
