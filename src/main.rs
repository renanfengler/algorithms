use std::{io, cmp::Ordering};

fn main() {
    println!("Number algorithms in Rust");
    println!("Insert how many fibonacci number to calculate");

    let mut count = String::new();
    io::stdin().read_line(&mut count).expect("Insert a valid number");
    let count: usize = count.trim().parse().expect("Could not parse input test");

    fibonacci(&count);
}

pub fn fibonacci(count: &usize) {
    let mut i = 1;
    let mut vector: Vec<usize> = Vec::new();

    vector.push(0);
    vector.push(1);

    loop {
        match i.cmp(&count) {
            Ordering::Equal => break,
            Ordering::Greater => break,
            Ordering::Less => {
                let last = vector[i];
                let second_to_last = vector[i - 1];
                let current = last + second_to_last;

                vector.push(current);
                i += 1;
            },
        }
    }

    for x in vector.iter() {
        println!("{}", x);
    }
}
