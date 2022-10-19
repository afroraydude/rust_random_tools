use std::io;
use super::objs::{Tile, Board};



pub fn play() {
    println!("Welcome to Minesweeper!");
    println!("Enter the size of the board (e.g. 10x10): ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    let mut input = input.split("x");
    let width: usize = input.next().unwrap().parse().unwrap();
    let height: usize = input.next().unwrap().parse().unwrap();

    println!("Enter the number of mines: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mines: usize = input.trim().parse().unwrap();

    let mut board = Board::new(width, height, mines);

    while !board.is_win() {
        board.print_board();
        println!("Enter a command (e.g. select 5 5 or flag 5 5): ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        let mut input = input.split(" ");
        let command = input.next().unwrap();
        let x: usize = input.next().unwrap().parse().unwrap();
        let y: usize = input.next().unwrap().parse().unwrap();

        if (x >= width || y >= height) {
            println!("Invalid coordinates");
            continue;
        }

        if command == "select" {
            if board.select_tile(x, y) {
                println!("You hit a mine! Game over!");
                board.on_lost();
                break;
            }
        } else if command == "flag" {
            board.flag_tile(x, y);
        } else {
            println!("Invalid command");
        }
    }
}
