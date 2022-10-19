use std::io;

mod random;
mod minesweeper;

fn main() {
    println!("Tools: ");
    println!("1. Test indev tool");
    println!("2. Random from list");
    println!("3. Minesweeper");


    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    // convert to int
    let input: i32 = input.parse().unwrap();

    match input {
        1 => println!("No tool indev"),
        2 => random::random_from_list(),
        3 => minesweeper::play_game(),
        _ => println!("Invalid input"),
    }
}
