#[derive(Copy, Clone)]
enum Player {
    Player1,
    Player2,
}

fn print_board(board: &[[char; 3]; 3]) {
    for item_x in board {
        for item_y in item_x {
            print!("{} ", item_y);
        }
        println!();
    }
}

fn take_turn(player: Player, board: &mut [[char; 3]; 3]) -> Player {
    use text_io::read;

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

    match player {
        Player::Player1 => {
            board[y_pos][x_pos] = 'X';
            Player::Player2
        }
        Player::Player2 => {
            board[y_pos][x_pos] = 'O';
            Player::Player1
        }
    }
}

fn has_won() -> bool {
    false
}

fn main() {
    let mut cur_player = Player::Player1;
    let mut board: [[char; 3]; 3] = [['_', '_', '_'], ['_', '_', '_'], ['_', '_', '_']];


    while !has_won() {
        cur_player = take_turn(cur_player, &mut board);
        print_board(&board);
    }
}
