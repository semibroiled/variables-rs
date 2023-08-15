fn main() {
    // Mutable variables
    let x = 5;
    println!("x is {x}");

    let mut x = 7;
    println!("x is {x}");

    x = 6;
    println!("x is {x}");

    // Constants
    const ONE_HOUR_IN_SECONDS : u32 = 60*60;

    //const X_HOUR_IN_SECONDS : u32 = x*60*60; //error

    // Shadowing; already saw with x

    let x = ONE_HOUR_IN_SECONDS +1 ;

    println!("x is {x}");

    let x = x -2 ;
    println!("x is {x}");

    {
        let x = x * ONE_HOUR_IN_SECONDS;
        println!("x is {x}");
    }

    println!{"x is {x}"};

    let spaces = "    "; // string type

    let spaces = spaces.len(); // now store as length of spaces // usize

    println!("{spaces}"); //   println!(spaces) not allowed

    //alternative that is bad

    //let mut spaces = "   ";
    //spaces = spaces.len(); // wont compile because space expects string, but we now want it to be num type
    //mutating variable types is not allowed

    // Data Types

    let number_from_str: u32 = "100".parse().expect("Not a number");
    println!("{number_from_str}");

    //or
    /* 
    let number_from_str: u32 =  match "120".parse() {
        Ok(number) => number,
        Err(_) => _, //Huh, not working because of this for some reason
    }; */

    let num : u32 = 1000;
    println!("{num}");

    let num = 1_000_f64;
    println!("{num}");


    let num  = 0xffu32;
    println!("{num}");


    let num  = 0b1111_000;
    println!("{num}");
    

    println!("{number_from_str}");

    // Addition, subtraction, multiplication, divisoin, remainder

    let sum = 10+5;
    println!("{sum}");
    let diff = 10-5;
    println!("{diff}");
    let prod = 10*5;
    println!("{prod}");
    let quot = 56.1/8.6;
    let quot_int = 56/10;
    println!("{quot}, {quot_int}");
    let rem = 50%4;
    println!("{rem}");
    
    // Tuple
    let tup: (i32, f64, char) = (500, 6.4, 'a');

    let (x,y,z) = tup;

    println!("{},{},{}", x,y,z);

    let y = tup.1;

    println!("{y}");

    // Arrays

    let arr: [i32; 4] = [1,2,3,4];
    println!("{}", arr[1]);

    let arr = [10;20];

    for i in 0..arr.len() {

        println!("{} : {}", i, arr[i]);

    }

    //Functions

    another_function();
    



}

// Functions
fn another_function() {
    println!("This is another fcn");
}