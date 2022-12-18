fn main() {
    let _x: u8 = 9; // Range: (0) till (255), if we put a value above 255 the compiler will return an error.
    let _y: i8 = 10; //Range: (-128) till (127), if we put a value below -128 and/or above 127 the compiler will return an error.
                     // this won't work because _x is an unsigned integer, and _y is a signed integer, to add any type of numbers, we need to make sure the type of the integers/floats is the same.
                     // you can test this by uncommenting the 2 lines below:
                     //let _z = _x + _y
                     //println!("summed values of x and y: {}", _z);

    // this code below will work since the type of the integers are the same.
    let x_v2: u8 = 9;
    let y_v2: u8 = 10;

    // printing both values.

    // Remember: adding any 2 types of values weather it be u8 or u64, it will give an error, make sure to ALWAYS do math or anything related to variables with one single type of variable related to the respective task.

    let z = x_v2 + y_v2;

    println!("The added value of X and Y is: {}", z);
}
