use std::cmp::Ordering;

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

