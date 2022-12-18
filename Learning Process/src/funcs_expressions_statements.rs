// main function
fn main() {
    // to use the function, we just type it in the main functions
    test();
    // it is also possible to reuse these functions
    test();
    add_number(20, 30)
}
// we can also create our own functions:
// it can only be used in snake case
fn test() {
    println!("Test has been called");
}

// this function below, takes 2 parameters, called x and y as a 32 bit integer
// it then requires for the parameters to be specified when the function is called.
// for eg, add_number(10, 20);
// this commented snippet will print 30 as an answer
fn add_number(x: i32, y: i32) {
    println!("This sum is: {}", x + y);
}

// Note: Rust functions can return expressions but not statements

fn statements() {
    // this doesn't evluate something, so we can't give is another value over the value again
    // eg, let y = (let x = 20); <-- Error prone code
    // we also can't allow variables to contain functions, it would return an error
    // statements don't return anything, they don't evaluate.
    // the snippet of code below is a statement
    let x = 20;
}
fn expressions() {
    // an expression is pretty much anything else in rust that performs evaluation
    // for eg, functions, macros, arithmetic params, and etc. Are all expressions
    // for eg, 2 < 3;
    // this below is an expression, which evluates to 4
    //
    let number = {
        let x = 3;
        // we don't put a semicolon here, because I'm actually returning the value 4 from the expression above.
        // if we do put a semicolon here, rust will give an error saying that the expression is not returning a value
        // thus not being able to print it, basically the semicolon will make the whole thing a statement
        x + 1
    };

}
// this will return the value of x and y in the same manner
// but unlike the other one we can store it in a variable
// for eg,
// let result = add_numbers(2, 3);
// println!("{}", result);
// we can also use the return keyword to use a semicolon
fn add_numbers(x: i32, y: i32) -> i32 {
    // if we put a semicolon below it won't work
    x + y
    // return version
    return x + y;
}
