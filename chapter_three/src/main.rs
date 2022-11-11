

fn main() {
    let mut x = 5;
    println!("The value of x = {x}");
    x = 6;
    println!("The value of x = {x}");
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x = {x}");

    // shadowing, statically typed language 
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces = {spaces}");



     // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    // python rise up
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let tup_2: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup_2.0;

    let six_point_four = tup_2.1;

    let one = tup_2.2;

    let array = [1,2,3,4,5];
    let array = [3; 5]; // [3,3,3,3,3]
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array index = {}",array[0]); // why cant i have {array[0]}????

    new_function();
    function_with_param(five());
    
    if x > 5 {
        println!("x > 5");
    } else if x < 5 {
        println!("x < 5");
    } else if x == 5 {
        println!("x == 5");
    } else {
        println!("impossible");
    }

    for element in array {
        println!("element = {element}");
    }

    for n in (1..4).rev() { // inclusive..exclusive
        println!("{n}!");
    }



}

fn new_function() {
    println!("hello from another function!");
}

fn function_with_param(x: i32) {
    println!("x = {x}");
}

fn five() -> i32 {
    4 // you can do this with no semi colon,
      //  but if you do 4; it will cause an error
      // preferably do return 4;
}

