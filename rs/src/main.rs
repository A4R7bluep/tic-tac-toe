use std::fmt;
use termion::color;

// #[derive(Copy, Clone)]
#[derive(Debug)]
enum Player {
    Player1,
    Player2,
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn print_board(board: &[[char; 3]; 3]) {
    for item_x in board {
        for item_y in item_x {
            print!("{} ", item_y);
        }
        println!();
    }
}

fn take_turn(cur_player: &mut Player, board: &mut [[char; 3]; 3]) {
    use termion::color;
    use text_io::read;

    println!(
        "{}{}'s turn{}",
        color::Fg(color::LightBlue),
        cur_player,
        color::Fg(color::Reset)
    );
    print!("X position (0 to 2): ");
    let mut x_pos: usize = read!();

    while x_pos > 2 {
        print!("Incorrect input - try again");
        print!("X position (0 to 2): ");
        x_pos = read!();
    }

    print!("Y position (0 to 2): ");
    let mut y_pos: usize = read!();

    while y_pos > 2 {
        print!("Incorrect input - try again");
        print!("Y position (0 to 2): ");
        y_pos = read!();
    }

    match cur_player {
        Player::Player1 => {
            board[y_pos][x_pos] = 'X';
            *cur_player = Player::Player2;
        }
        Player::Player2 => {
            board[y_pos][x_pos] = 'O';
            *cur_player = Player::Player1;
        }
    }
}

fn has_won(board: &[[char; 3]; 3]) -> bool {
    // (0, 0); (1, 0); (2, 0)
    if board[0][0] == board[1][0] && board[1][0] == board[2][0] && board[0][0] != '_' {
        return true;
    }
    // (0, 1); (1, 1); (2, 1)
    if board[0][1] == board[1][1] && board[1][1] == board[2][1] && board[0][1] != '_' {
        return true;
    }
    // (0, 2); (1, 2); (2, 2)
    if board[0][2] == board[1][2] && board[1][2] == board[2][2] && board[0][2] != '_' {
        return true;
    }
    // (0, 0); (0, 1); (0, 2)
    if board[0][0] == board[0][1] && board[0][1] == board[0][2] && board[0][0] != '_' {
        return true;
    }
    // (1, 0); (1, 1); (1, 2)
    if board[1][0] == board[1][1] && board[1][1] == board[1][2] && board[1][0] != '_' {
        return true;
    }
    // (2, 0); (2, 1); (2, 2)
    if board[2][0] == board[2][1] && board[2][1] == board[2][2] && board[2][0] != '_' {
        return true;
    }
    // (0, 0); (1, 1); (2, 2)
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != '_' {
        return true;
    }
    // (0, 2); (1, 1); (2, 0)
    if board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] != '_' {
        return true;
    }
    
    false
}

fn main() {
    let mut cur_player: Player = Player::Player1;
    let mut board: [[char; 3]; 3] = [
        ['_', '_', '_'], 
        ['_', '_', '_'], 
        ['_', '_', '_']];

    while !has_won(&board) {
        take_turn(&mut cur_player, &mut board);
        print_board(&board);
    }

    println!(
        "{}{} won!{}",
        color::Fg(color::LightGreen),
        cur_player,
        color::Fg(color::Reset)
    );
}
