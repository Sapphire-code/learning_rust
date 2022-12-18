use std::io;

fn main() {
    println!("Enter your choice below");
    println!("Your Options are:");
    println!("1. add");
    println!("2. subtract");
    println!("3. multiply");
    println!("4. divide");
    let mut input: String = String::new();
    
    io::stdin().read_line(&mut input);

    match input.trim() {
        "add" => {
            let mut digit_1 = String::new();
            std::io::stdin().read_line(&mut digit_1).unwrap();
            let number_1: i32 = digit_1.trim().parse().unwrap();
            let mut digit_2 = String::new();
            std::io::stdin().read_line(&mut digit_2).unwrap();
            let number_2: i32 = digit_2.trim().parse().unwrap();
            add(number_1, number_2);
        }
        "subtract" => {
            let mut digit_1 = String::new();
            std::io::stdin().read_line(&mut digit_1).unwrap();
            let number_1: i32 = digit_1.trim().parse().unwrap();
            let mut digit_2 = String::new();
            std::io::stdin().read_line(&mut digit_2).unwrap();
            let number_2: i32 = digit_2.trim().parse().unwrap();
            subtract(number_1, number_2);
        }
        "multiply" => {
            let mut digit_1 = String::new();
            std::io::stdin().read_line(&mut digit_1).unwrap();
            let number_1: i32 = digit_1.trim().parse().unwrap();
            let mut digit_2 = String::new();
            std::io::stdin().read_line(&mut digit_2).unwrap();
            let number_2: i32 = digit_2.trim().parse().unwrap();
            multiply(number_1, number_2);
        }
        "divide" => {
            let mut digit_1 = String::new();
            std::io::stdin().read_line(&mut digit_1).unwrap();
            let number_1: i32 = digit_1.trim().parse().unwrap();
            let mut digit_2 = String::new();
            std::io::stdin().read_line(&mut digit_2).unwrap();
            let number_2: i32 = digit_2.trim().parse().unwrap();
            divide(number_1, number_2);
        }
        _ => {
            println!("Improper input, please enter the options that have been given");
        }
    }
}
fn add(xa: i32, ya: i32) -> i32 {
    println!("{}", xa + ya);
    return xa + ya
}
fn subtract(xs: i32, ys: i32) -> i32 {
    println!("{}", xs - ys);
    return xs - ys;
}
fn multiply(xm: i32, ym: i32) -> i32 {
    println!("{}", xm * ym);
    return xm * ym;
}
fn divide(xd: i32, yd: i32) -> i32 {
    println!("{}", xd / yd);
    return xd / yd;
}
