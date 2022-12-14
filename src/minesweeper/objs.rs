pub struct Tile {
    pub is_mine: bool,
    pub is_revealed: bool,
    pub is_flagged: bool,
    pub adjacent_mines: u8,
}

pub struct Board {
    tiles: Vec<Vec<Tile>>,
    width: u8,
    height: u8,
    mines: u8,
}

impl Board {
    pub fn new(width: usize, height: usize, mines: usize) -> Board {
        let mut tiles = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Tile {
                    is_mine: false,
                    is_revealed: false,
                    is_flagged: false,
                    adjacent_mines: 0,
                });
            }
            tiles.push(row);
        }

        let mut board = Board {
            tiles: tiles,
            width: width as u8,
            height: height as u8,
            mines: mines as u8,
        };

        board.place_mines();
        board.calculate_adjacent_mines();

        board
    }

    fn place_mines(&mut self) {
        let mut mines_placed = 0;
        while mines_placed < self.mines {
            let x = rand::random::<usize>() % self.width as usize;
            let y = rand::random::<usize>() % self.height as usize;

            if !self.tiles[y][x].is_mine {
                self.tiles[y][x].is_mine = true;
                mines_placed += 1;
            }
        }
    }

    fn calculate_adjacent_mines(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut adjacent_mines = 0;
                for y_offset in -1..=1 {
                    for x_offset in -1..=1 {
                        if x_offset == 0 && y_offset == 0 {
                            continue;
                        }

                        let x = x as i8 + x_offset;
                        let y = y as i8 + y_offset;

                        if x < 0 || x >= self.width as i8 || y < 0 || y >= self.height as i8 {
                            continue;
                        }

                        if self.tiles[y as usize][x as usize].is_mine {
                            adjacent_mines += 1;
                        }
                    }
                }
                self.tiles[y as usize][x as usize].adjacent_mines = adjacent_mines;
            }
        }
    }

    pub fn print_board(&self) {
        // the board should be printed like this:
        /*
             1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
             -----------------------------------
          1|
          2|
          3|
          4|
          5|
          6|
          7|
          8|
          9|
         10|
         11|
         12|
         13|
         14|
         15|
        */
        // print the prepending spaces
        for _ in 0..self.height.to_string().len() {
            print!(" ");
        }
        print!(" ");
        let mut string = String::new();
        for x in 0..self.width {
            string.push_str(&format!("{} ", x));
        }
        println!("{}", string);

        // print the horizontal line
        for _ in 0..self.height.to_string().len() {
            print!(" ");
        }
        print!(" ");
        for _ in 0..string.len() - 1 {
            print!("-");
        }
        println!();

        // print the board with the numbers
        for y in 0..self.height {
            print!("{:width$}", y, width = self.height.to_string().len());
            print!("|");
            for x in 0..self.width {
                let tile = &self.tiles[y as usize][x as usize];
                if tile.is_revealed {
                    if tile.is_mine {
                        print!("*");
                    } else {
                        print!("{}", tile.adjacent_mines);
                    }
                } else if tile.is_flagged {
                    print!("F");
                } else {
                    print!(".");
                }

                // print the spaces betweem based on the number of digits in the next number
                let next_x = (x + 1).to_string().len();

                for _ in 0..next_x {
                    print!(" ");
                }
            }
            println!();
        }
    }

    pub fn select_tile(&mut self, x: usize, y: usize) -> bool {
        let tile = &mut self.tiles[y][x];
        if tile.is_revealed {
            return false;
        }

        tile.is_revealed = true;
        if (tile.is_mine) {
            return true;
        }

        if tile.adjacent_mines == 0 {
            for y_offset in -1..=1 {
                for x_offset in -1..=1 {
                    if x_offset == 0 && y_offset == 0 {
                        continue;
                    }

                    let x = x as i8 + x_offset;
                    let y = y as i8 + y_offset;

                    if x < 0 || x >= self.width as i8 || y < 0 || y >= self.height as i8 {
                        continue;
                    }

                    if self.select_tile(x as usize, y as usize) {
                        println!("Mine found at {}, {}", x, y);
                    }
                }
            }
        }
        false
    }

    pub fn flag_tile(&mut self, x: usize, y: usize) {
        let tile = &mut self.tiles[y][x];
        if tile.is_revealed {
            println!("Tile is already revealed");
            return;
        }

        tile.is_flagged = !tile.is_flagged;
    }

    pub fn is_win(&self) -> bool {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = &self.tiles[y as usize][x as usize];
                if !tile.is_mine && !tile.is_revealed {
                    return false;
                }
            }
        }
        true
    }

    pub fn on_lost(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = &mut self.tiles[y as usize][x as usize];
                if tile.is_mine {
                    tile.is_revealed = true;
                }
            }
        }

        self.print_board();
    }
}
