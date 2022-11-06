use std::io;

mod blackjack;
mod fib;
mod grader;
mod minesweeper;
mod random;

fn main() {
    println!("Tools: ");
    println!("1. Fibonacci");
    println!("2. Random from list");
    println!("3. Minesweeper");
    println!("4. Blackjack");
    println!("5. Grade Calculator");
    println!("6. Magic 8 Ball");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    // convert to int
    let input: i32 = input.parse().unwrap_or_else(|_| {
        println!("Input not recognized");
        0
    });

    match input {
        1 => fib::fib(),
        2 => random::random_from_list(),
        3 => minesweeper::play_game(),
        4 => blackjack::play(),
        5 => grader::grader(),
        6 => random::magic_ball(),
        _ => println!("Invalid input"),
    }
}
