fn main() {
    // primitive data types are fundamental data types that are used to declare a variable.

// Scalar Data Type: a scalar data type is something that has a finite set of possible values, following some scale, i.e. each value can be compared to any other value as either, equal, or less.

// Compound data type: in computer science a composite data type or a compound data type is any data type which can be constructed in a program using the programming language's primitive data types and other composite types.

// -- SCALAR VARIABLES START --

// INTEGERS
// i means signed and u means unsigned
// options for signed variables:
// i8, i16, i32, i64, i128.
// options for unsigned variables:
// u8, u16, u32, u64, u128.
// also unsigned integers can't have a negative sign in them.
let x: i32 = 5;
let y: u32 = 25;

println!("x is a signed integer: {}", x);
println!("y is an unsigned integer {}", y);

//FLOATING POINT NUMBERS
// basically numbers with decimal points, eg. 18.92.

// there are some new assinging techniques here.
// f32, for single precision or just 1 number after the decimal place.
// f64, for double precision or just 2 numbers after the decimal place.

// f32 example:
let single_decimal_number: f32 = 15.9;

// f64 example:
let double_decimal_number: f64 = 15.99;

println!("Single Decimal Number: {}", single_decimal_number);
println!("Double Decimal Number {}", double_decimal_number);

// BOOLEANS
// Booleans are just variables with only 1 of 2 values, either true or false.
// we can also strictly specify bools, for eg. let bool_value: bool = true
// we can also use 0 for false and 1 for true.

let bool1 = true;
let bool2 = false;

// 0 means false
let bool3 = 0;
// 1 means true
let bool4 = 1;

println!("bool 1: {}", bool1);
println!("bool 2: {}", bool2);
println!("bool 3: {}", bool3);
println!("bool 4: {}", bool4);

// CHARACTER
// Character data types just store single characters in them.

let my_char: char = 'A';
println!("my_char is: {}", my_char);

// -- SCALAR VARIABLES END --
// ==============================
// -- COMPUND VARIABLES START --

// There are 2 types of compund variables, the 2 being tuples and arrays.

// - TUPLES -
// a tuple is a fixed lenth of sequence of elements that is immutable, we can put any values we in in tuples, bool, numbers & floats, chars and all of the above.

// we can also implicitly specify which type of tuple this is.
let mut my_tuple: (i32, bool, char) = (1, true, 'x');

println!("My tuple: {:?}", my_tuple);

// accessing individual elements of a tuple
println!("Tuple Object 1: {}", my_tuple.0);
println!("Tuple Object 2: {}", my_tuple.1);
println!("Tuple Object 3: {}", my_tuple.2);

// Modifying individual elements of a tuple, we cannot add elements to the tuple
my_tuple.0 = 10;
my_tuple.1 = false;
my_tuple.2 = 'y';
// printing the tuple to check if the modification has worked or not
println!("My tuple - Modified: {:?}", my_tuple);

// - ARRAY -
// Arrays can only have a single type of element in them, if we try to keep multiple types of elements we will receive an error by the compiler. we also cannot add elements into an array unlike other languages

// declaring an array.
// we can also implicitly specify what type of data is in the array
let mut my_array: [i32; 5] = [1, 2, 3, 4, 5];

//
println!("The array is: {:?}", my_array);

// accessing each element of the array
println!("The elements of the array is: ");
println!("{}", my_array[1]);
println!("{}", my_array[2]);
println!("{}", my_array[3]);
println!("{}", my_array[4]);

// Modifying each element of the array
println!("The modified elements of the array is: ");
my_array[0] = 2;
my_array[1] = 3;
my_array[2] = 4;
my_array[3] = 5;
my_array[4] = 6;

println!("{:?}", my_array);
}
