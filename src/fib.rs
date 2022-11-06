use std::fmt::format;
use std::io;

fn fib_internal(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib_internal(n - 1) + fib_internal(n - 2);
    }
}

pub fn fib() {
    // ask how long the sequence should be
    println!("How long should the sequence be?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    // convert to int
    let input: u32 = input.parse().unwrap_or_else(|_| {
        println!("Invalid input");
        0
    });

    // print the sequence
    let mut sequence = String::new();
    for i in 0..input {
        sequence.push_str(&format!("{}", fib_internal(i)));
        sequence.push_str(" ");
    }
    println!("{}", sequence);
}
