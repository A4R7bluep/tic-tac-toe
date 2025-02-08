use termion::color;

fn print_board(board: &[[char; 3]; 3]) {
    use termion::color;
    println!("{}Welcome to Tic-Tac-Toe!{}", color::Fg(color::LightGreen), color::Fg(color::Reset));
    println!();
    for item_x in board {
        for item_y in item_x {
            print!("{} ", item_y);
        }
        println!();
    }
    println!();
}

fn take_turn(turn_num: i32, board: &mut [[char; 3]; 3]) {
    use termion::color;
    use text_io::read;

    println!(
        "{}Player {}'s turn{}",
        color::Fg(color::LightBlue),
        (turn_num % 2) + 1,
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

    match turn_num % 2 {
        0 => {
            if board[y_pos][x_pos] == '_' {
                board[y_pos][x_pos] = 'X';
            } else {
                println!("\n{}Area not empty{}", color::Fg(color::Red), color::Fg(color::Reset));
                take_turn(turn_num, board);
            }
        }
        1 => {
            if board[y_pos][x_pos] == '_' {
                board[y_pos][x_pos] = 'O';
            } else {
                println!("\n{}Area not empty{}", color::Fg(color::Red), color::Fg(color::Reset));
                take_turn(turn_num, board);
            }
        }
        _ => {
            println!("Erroneous turn number");
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
    let mut turn_num: i32 = 0;
    let mut board: [[char; 3]; 3] = [['_', '_', '_'], ['_', '_', '_'], ['_', '_', '_']];

    print_board(&board);

    while !has_won(&board) {
        take_turn(turn_num, &mut board);
        print_board(&board);
        turn_num += 1;
    }

    println!(
        "{}Player {} won!{}",
        color::Fg(color::LightGreen),
        turn_num % 2,
        color::Fg(color::Reset)
    );
}
