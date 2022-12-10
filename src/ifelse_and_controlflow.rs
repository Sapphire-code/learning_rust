fn main() {
    // conditional operators
    // <
    // >
    // <=
    // >=
    // !=
    // ==

    // this will work, because 2 and 3 are the same data type.
    let cond_a = 2 < 3;
    println!("{}", cond_a);
    // this code will produce errors, so to fix this we can implement the code below this broken code
    // this code will produce errors due to the numbers being different data types
    let cond_b = 2 < 2.2;
    println!("{}", cond_b)
    // correct code
    let cond_c = (2 as f32) < 2.2;
    println!("{}", cond_c)

    // compund conditions
    // multiple conditions chained together
    // && - and
    // || - or
    // ! - not

    // eg, this will print out true even if 1 of them is false then it will print false:
    let cond_d = (true && true);
    println!("{}", cond_d);

    // eg, this will print out true if one of the statements is true, but if both are false then it will print false:
    let cond_e = (false || true);
    println!("{}", cond);

    // eg, this code below will make the bool value if true to false and if false to true:

    let cond_f = !(true || true);
    println!("{}", cond2)

    // flow of these compund operators are:
    // first, !
    // second, &&
    // third ||

    // control flow.

    let food = "cookie";
    
    if (food == "cookie") {
        println!("I like cookies too!");
    }
    if (food != "cookie") {
        println!("You don't like cookies? Damn.");
    }
    if (food == ("cookie" || "burger") {
        println!("food is either cookie or burger");
    } else {
        println!("Cookie is something else");
    } else if (food == "fruit") {
        println!("That's healthy, yummy");
    } else if (food == "bread") {
        println!("that's boring");
    }
}