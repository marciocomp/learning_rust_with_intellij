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

    /************NUMERIC OPERATIONS************/

    let a = 10;
    let b = 5.5555;
    //let c = a  + b ; //It is not possible. the type-conversion must be explicit as the next line
    let sum: f64 = (a as f64) + b;
    println!("Sum: {}",sum);
    let sum: i32 = a + (b as i32);
    println!("Sum: {}",sum);

    let c= b as i32;//truncated
    println!("C: {}",c);

    let difference = b - (a as f64);
    println!("Difference: {}",difference);

    let product = (c as f64) * b;
    println!("Product: {}", product);

    let quotient = 50/12;//integer division
    println!("Quotient: {}", quotient);

    let quotient = 50.0/12.0;//floating-point division
    println!("Quotient: {}", quotient);

    let remainder = a % 3;
    println!("Remainder: {}",remainder);

    let remainder = (a + 2) % 3;
    println!("Remainder: {}",remainder);


    /************END NUMERIC OPERATIONS************/

    /************BOOLEAN************/

    let learning_rust = true;
    let give_up: bool = false;
    let success = learning_rust && !give_up;// A simple boolean operation
    println!("Success: {}", success); // ;)

    /************END BOOLEAN************/

    /************CHARACTER************/

    let c = 'A';
    let b: char = '\u{221a}';// Code to square root
    let z = '\u{221b}'; // Code to cube root
    let emoji: char  = '\u{1f913}';//To be a nerd :)

    println!("C: {:X}", c as u32);// It prints the code of the table Unicode
    println!("B: {}", b);
    println!("Z: {}", z);
    println!("Emoji: {}",emoji);

    /************END CHARACTER************/


    /************TUPLE TYPES************/
    // Tup: (id, [last name, first name], birth (mm,dd,yyyy))
    let tup: (u128,[&str;2],(u8,u8,u64)) = (12345678,["Lopes","Marcio"],(10,14,1983));
    let tup1 = (12345678,["Lopes","Marcio"],(10,14,1983));
    let (x,y,z) = tup;

    println!("X: {:?}",x);
    println!("y: {:?}",y);
    println!("z: {:?}",z);

    println!("Tup: {:?}", tup);
    println!("Tup1.1: {:?}",tup1.1);

    /************END TUPLE TYPES************/

    /************ARRAYS************/

    let numbers = [1,2,3,4];//The size is defined by total of elements in the []
    println!("Numbers: {:?}",numbers);

    let numbers: [i32;20] = [0;20];//The size is defined by annotation type
                                   //It completes all positions with 0 (zero)
    println!("Numbers: {:?}",numbers);

    let names = ["Marcio", "Lopes"];
    println!("Names: {:?}",names);

    let names: [&str; 20] = ["name";20];
    println!("Names: {:?}",names);

    /************END ARRAYS************/


}
