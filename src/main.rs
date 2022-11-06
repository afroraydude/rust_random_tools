use std::io;

mod blackjack;
mod fib;
mod grader;
mod minesweeper;
mod random;

fn main() {
    let choice: i32;

    // grab args
    let args: Vec<String> = std::env::args().collect();
    let mut option = String::new();
    
    if args.len() > 1 {
        option = args[1].clone();
    }

    let option: i32 = option.trim().parse().unwrap_or_default();

    if option > 0 && option < 7 {
        choice = option;
    } else {
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
        choice = input;
    }

    match choice {
        1 => fib::fib(),
        2 => random::random_from_list(),
        3 => minesweeper::play_game(),
        4 => blackjack::play(),
        5 => grader::grader(),
        6 => random::magic_ball(),
        _ => println!("Invalid option"),
        _ => println!("Invalid input"),
    }
}
