fn main() {
    // Prelude:
    // A Prelude is a list of things that rust automatically imports into every rust program.
    // it's kept as small as possible, and is focused on things particularly traits, which are
    // used in almost every single rust program.

    // :: is the path separator, for eg. "use std::io;" the std library is the main library and io is a certain place in the std library.

    let mut input = String::new();
    // calling the standard input function from the io module, then reading the line with ".read_line(&mut input) to allow the function to access the input value and allow the function to modify it. Then we use a .expect() function to make sure that is catches any errors that occur."
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    println!("The user has decreed: {}", input);
}
