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
    pub board: [char; 9],
    pub turn: u8,
}

impl Game {
    pub fn new() -> Game {
        println!("Welcome to Tic Tac Toe!\n\n");
        println!("Player 1: X");
        println!("Player 2: O\n\n");
        Game {
            board: [' '; 9],
            turn: 1,
        }
    }

    pub fn print_board(&self) {
        println!(" {} | {} | {}", self.board[0], self.board[1], self.board[2]);
        println!("---+---+---");
        println!(" {} | {} | {}", self.board[3], self.board[4], self.board[5]);
        println!("---+---+---");
        println!(" {} | {} | {}", self.board[6], self.board[7], self.board[8]);
    }

    pub fn play_game(&mut self) {
        let mut turn = self.turn - 1;
        while turn < 10 {
            println!("Player {} turn: ", self.turn);
            let input = loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                match input.trim().parse::<usize>() {
                    Ok(num) if self.is_valid_move(num) => break num,
                    _ => println!("Please enter a valid number between 1 and 9 that has not been taken: "),
                }
            };

            self.board[input - 1] = match self.turn {
                1 => 'X',
                2 => 'O',
                _ => '_',
            };
            turn += 1;
            self.turn = turn.rem_euclid(2) + 1;
            
            self.print_board();

            let win_state = self.check_win();

            if win_state.1 {
                println!("{} wins!\n", win_state.0);
                break;
            } else if turn == 9 {
                println!("It's a draw!\n");
                break;
            } 
            
        }
    }

    fn is_valid_move(&self, input: usize) -> bool {
        let valid_num = input >= 1 || input <= 9;
        let unused_space = self.board[input - 1] != 'X' && self.board[input - 1] != 'O';
        valid_num && unused_space
    }

    fn check_win(&self) -> (char, bool) {
        let mut win = false;
        let mut game_char = ' ';
        
        // Check rows
        for i in 0..3 {
            if self.board[i * 3] != ' ' && self.board[i * 3] == self.board[i * 3 + 1] && self.board[i * 3 + 1] == self.board[i * 3 + 2] {
                win = true;
                game_char = self.board[i * 3];
                break;
            }
        }
        
        // Check columns
        for i in 0..3 {
            if self.board[i] != ' ' && self.board[i] == self.board[i + 3] && self.board[i + 3] == self.board[i + 6] {
                win = true;
                game_char = self.board[i];
                break;
            }
        }
    
        // Check diagonals
        if self.board[0] != ' ' && self.board[0] == self.board[4] && self.board[4] == self.board[8] {
            win = true;
            game_char = self.board[0];
        }
        
        if self.board[2] != ' ' && self.board[2] == self.board[4] && self.board[4] == self.board[6] {
            win = true;
            game_char = self.board[2];
        }
        
        (game_char, win)
    }
    
}