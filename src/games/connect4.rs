use std::io;
use crate::command_line_tasks;


pub fn start_game() {
    let mut game = Game::new();
    command_line_tasks::clear_terminal();
    game.print_board();
    game.play_game();

    println!("Would you like to play again?\n\n");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim().to_lowercase();
    if input.chars().next() == Some('y') {
        start_game();
    } else {
        println!("Thanks for playing!\nGoodbye");
    }
}


struct Game {
    pub board: [char; 42],
    pub turn: u8,
    columns: [usize; 7],
}

impl Game {
    pub fn new() -> Game {
        println!("Welcome to Connect-4!\n\n");
        println!("Player 1: X");
        println!("Player 2: O\n\n");
        Game {
            board: [' '; 42],
            turn: 1,
            columns: [0; 7],
        }
    }

    pub fn print_board(&self) {
        for i in (0..6).rev() {
            let mut row_string = String::new();
            
            row_string.push_str(" | ");
            for j in 0..7 {
                row_string.push_str(&self.board[i * 7 + j].to_string());
                row_string.push_str(" | ");
            }
            println!(" {}", row_string);
            println!("   ---+---+---+---+---+---+---");
        }
    }

    pub fn play_game(&mut self) {
        while self.turn <= 42 {
            println!("Player {} turn: ", if self.turn % 2 == 1 { 1 } else { 2 });

            let input = loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                match input.trim().parse::<usize>() {
                    Ok(num) if self.is_valid_move(num) => break num,
                    _ => println!("Please enter a valid number between 1 and 7 that has not been taken: "),
                }
            };

            let col = input - 1;
            let row = self.columns[col];
            self.board[row * 7 + col] = match self.turn % 2 {
                0 => 'O',
                _ => 'X',
            };
            self.columns[col] += 1;
            self.turn += 1;
            self.print_board();

            let win_state = self.check_win();
            if win_state.1 {
                println!("{} wins!\n", win_state.0);
                break;
            } else if self.turn > 42 {
                println!("It's a draw!\n");
                break;
            }
        }
    }

    fn is_valid_move(&self, input: usize) -> bool {
        input >= 1 && input <= 7 && self.columns[input - 1] < 6
    }

    fn check_win(&self) -> (char, bool) {
        for i in 0..6 {
            for j in 0..7 {
                if self.board[i * 7 + j] != ' ' {
                    // Check horizontal
                    if j <= 3 && (0..4).all(|k| self.board[i * 7 + j + k] == self.board[i * 7 + j]) {
                        return (self.board[i * 7 + j], true);
                    }
                    // Check vertical
                    if i <= 2 && (0..4).all(|k| self.board[(i + k) * 7 + j] == self.board[i * 7 + j]) {
                        return (self.board[i * 7 + j], true);
                    }
                    // Check diagonal /
                    if i <= 2 && j <= 3 && (0..4).all(|k| self.board[(i + k) * 7 + j + k] == self.board[i * 7 + j]) {
                        return (self.board[i * 7 + j], true);
                    }
                    // Check diagonal \
                    if i <= 2 && j >= 3 && (0..4).all(|k| self.board[(i + k) * 7 + j - k] == self.board[i * 7 + j]) {
                        return (self.board[i * 7 + j], true);
                    }
                }
            }
        }
        (' ', false)
    }
}