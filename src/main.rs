use colored::Colorize;
use connectfour::board::Board;
use connectfour::piece::Piece;
use getch_rs::{Getch, Key};

fn main() {
    let mut board = Board::new();
    let mut turn = true;

    let g = Getch::new();
    loop {
        let piece = if turn { Piece::Red } else { Piece::Yellow };
        board.display(piece);

        let color = if turn { "🔴" } else { "🟡" };
        match g.getch() {
            Ok(Key::Left) => {
                board.go_left();
                continue;
            }
            Ok(Key::Right) => {
                board.go_right();
                continue;
            }
            Ok(Key::Char('\r')) => {}
            Ok(Key::Char('q')) | Ok(Key::Char('Q')) => break,
            Err(_) => break,
            _ => continue,
        }

        let col_idx = board.curr;

        if !board.check_col_availability(col_idx) {
            println!("Column is not valid!");
            continue;
        }

        let row_idx = board.get_available_idx(col_idx);
        board.add_piece(col_idx, piece);
        board.display(piece);

        if board.check_win(row_idx, col_idx) {
            println!("{} {} has won!", "Game over!".green(), color);
            break;
        }
        if board.get_available_columns().len() == 0 {
            println!("It's a tie!");
            break;
        }

        turn = !turn;
    }
}
