mod command_line_tasks;
mod games;

fn main() {
    games::tic_tac_toe::start_game();
    games::connect4::start_game();
}
