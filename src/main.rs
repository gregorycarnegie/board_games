mod command_line_tasks;
mod games;

fn main() {
    print!("Welcome to the Tic Tac Toe and Connect 4 games!\n");
    println!("
    What game would you like to play?
    1. Tic Tac Toe
    2. Connect 4
    ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let input = input.trim();

    match input.parse::<u8>() {
        Ok(1) => games::tic_tac_toe::start_game(),
        _ => games::connect4::start_game() 
    }
}
