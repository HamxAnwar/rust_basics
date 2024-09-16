use std::io;

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';

const BOARD_SIZE: usize = 3;
type Board = [[char; BOARD_SIZE]; BOARD_SIZE];
// type define an alias for an existing type.
// Above is a 2D array. [char; BOARD_SIZE] are the number of columns and BOARD_SIZE are the number of rows. So a 3 x 3 2D array.

fn initialize_board() -> Board {
    return [[' '; BOARD_SIZE]; BOARD_SIZE];
}

fn print_board(board: &Board) {
    for row in board {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn get_player_move(current_player: char, board: &Board) -> (usize, usize) {
    loop {
        let mut input = String::new();
        println!("Make your move player {}: (row column)", current_player);
        io::stdin().read_line(&mut input).unwrap();

        let coordinates: Vec<usize> = input
            .split_whitespace()
            .flat_map(str::parse::<usize>)
            .collect();

        if coordinates.len() == 2 {
            let (row, col) = (coordinates[0], coordinates[1]);
            if row < BOARD_SIZE && col < BOARD_SIZE && board[row][col] == ' ' {
                return (row, col);
            }
        }
        println!("Invalid input!");
    }
}

fn play_game() {
    let mut board = initialize_board();
    let mut current_player = PLAYER_X;

    loop {
        println!("Current board: ");
        print_board(&board);

        let (row, col) = get_player_move(current_player, &board);
        board[row][col] = current_player;

        if check_winner(current_player, &board) {
            println!("Player {} is the winner", current_player);
            break;
        }

        if check_draw(&board) {
            println!("The match is a draw!");
            break;
        }

        current_player = if current_player == PLAYER_X {
            PLAYER_O
        } else {
            PLAYER_X
        }
    }
}

fn check_winner(current_player: char, board: &Board) -> bool {
    for i in 0..BOARD_SIZE {
        if (board[i][0] == current_player)
            && (board[i][1] == current_player)
            && (board[i][2] == current_player)
        {
            return true;
        } else if (board[0][i] == current_player)
            && (board[1][i] == current_player)
            && (board[2][i] == current_player)
        {
            return true;
        } else if (board[0][0] == current_player)
            && (board[1][1] == current_player)
            && (board[2][2] == current_player)
        {
            return true;
        } else if (board[0][2] == current_player)
            && (board[1][1] == current_player)
            && (board[2][0] == current_player)
        {
            return true;
        } else {
            return false;
        }
    }
    true
}

fn check_draw(board: &Board) -> bool {
    for row in board {
        for cell in row {
            if *cell == ' ' {
                return false;
            }
        }
    }
    return true;
}

fn main() {
    println!("Welcome to tic-tac-toe game.");
    play_game();
}
