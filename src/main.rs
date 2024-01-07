use std::io;
mod math;

fn main() {
    println!("Number algorithms in Rust");

    // Create a way to choose the program to run, and then come back to the menu
    println!("Insert how many fibonacci number to calculate");

    let mut count = String::new();
    io::stdin().read_line(&mut count).expect("Insert a valid number");
    let count: usize = count.trim().parse().expect("Could not parse input test");

    let fib_result = math::math::fibonacci(&count);
    for x in fib_result.iter() {
        println!("{}", x);
    }
}

