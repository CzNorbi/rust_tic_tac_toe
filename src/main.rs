use std::io;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum FieldState {
    X,
    O,
    Empty
}

impl FieldState {
    fn to_string(&self) -> String {
        match *self {
          FieldState::X => "X".to_string(),
          FieldState::O => "O".to_string(),
          FieldState::Empty => " ".to_string()
        }
      }
}

fn get_board(board: &[FieldState; 9]) -> String {
    let mut board_str = String::new();
    board_str.push_str("-------------\n");
    for i in 0..3 {
        board_str.push_str(&format!(
            "| {} | {} | {} |\n",
            board[3 * i].to_string(),
            board[3 * i + 1].to_string(),
            board[3 * i + 2].to_string()
        ));
        board_str.push_str("-------------\n");
    }
    board_str
}

fn validate_step(step: usize, board: &[FieldState; 9]) -> bool {
    if step > 8 {
        return false;
    };
    if board[step] != FieldState::Empty {
        return false
    }
    return true
}

fn has_empty_fields(board: &[FieldState; 9]) -> bool {
    board.iter().any(|&state| matches!(state, FieldState::Empty))
}

fn has_won(player_symbol: FieldState, board: &[FieldState; 9]) -> bool {
    // row
    for i in 0..3 {
        if board[3*i] == player_symbol && board[3*i + 1] == player_symbol && board[3*i + 2] == player_symbol {
            return true;
        }
    }
    //column
    if board[0] == player_symbol && board[3] == player_symbol && board[6] == player_symbol {
        return true;
    }
    if board[1] == player_symbol && board[4] == player_symbol && board[7] == player_symbol {
        return true;
    }
    if board[2] == player_symbol && board[5] == player_symbol && board[8] == player_symbol {
        return true;
    }
    // diagonal
    if board[0] == player_symbol && board[4] == player_symbol && board[8] == player_symbol {
        return true;
    }
    if board[2] == player_symbol && board[4] == player_symbol && board[6] == player_symbol {
        return true;
    }
    return false
}

fn main() {
    println!("Game is starting...");

    let mut active_player = FieldState::X;
    let mut board = [FieldState::Empty, FieldState::Empty, FieldState::Empty,
                                      FieldState::Empty, FieldState::Empty, FieldState::Empty,
                                      FieldState::Empty, FieldState::Empty, FieldState::Empty];
    print!("{}", get_board(&board));

    loop {
        let mut step: usize = 0;

        loop {
            println!("Player '{}' Please input your move.", active_player.to_string());

            let mut input  = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse() {
                Ok(num) => {
                    if !validate_step(num, &board) {
                        println!("Invalid step!");
                        continue;
                    }
                    else {
                        step = num;
                        break;
                    }
                }
                Err(_) => {
                    println!("Please enter a valid number.");
                    continue;
                }
            }
        }

        board[step] = active_player;
        print!("{}", get_board(&board));

        if has_won(active_player, &board) {
            println!("{} has won!", active_player.to_string());
            break;
        }

        if !has_empty_fields(&board) {
            println!("Draw!");
            break;
        }

        active_player = match active_player {
            FieldState::X => FieldState::O,
            FieldState::O => FieldState::X,
            FieldState::Empty => FieldState::Empty
        }
    }
}
