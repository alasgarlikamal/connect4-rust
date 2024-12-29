pub use crate::piece::Piece;
use colored::Colorize;

#[derive(Debug)]
pub struct Board {
    columns: [[Piece; 6]; 7],
    pub curr: usize,
}

impl Board {
    pub fn new() -> Board {
        return Board {
            columns: [[Piece::Empty; 6]; 7],
            curr: 0,
        };
    }

    pub fn add_piece(&mut self, col_idx: usize, piece: Piece) {
        let row_idx = self.get_available_idx(col_idx);
        let col = self.columns.get_mut(col_idx).unwrap();
        col[row_idx] = piece;
    }

    pub fn check_col_availability(&self, col_idx: usize) -> bool {
        let col = self.columns[col_idx];
        match col[5] {
            Piece::Empty => return true,
            _ => return false,
        }
    }

    pub fn get_available_idx(&self, col_idx: usize) -> usize {
        let col = self.columns[col_idx];
        for (row_idx, piece) in col.iter().enumerate() {
            if let Piece::Empty = piece {
                return row_idx;
            }
        }
        unreachable!()
    }

    pub fn get_available_columns(&self) -> Vec<usize> {
        return (0..self.columns.len())
            .filter(|col_idx| self.check_col_availability(*col_idx))
            .collect();
    }

    pub fn check_win(&self, row_idx: usize, col_idx: usize) -> bool {
        fn compare_pieces(piece1: Piece, piece2: Piece) -> bool {
            match (piece1, piece2) {
                (Piece::Red, Piece::Red) => return true,
                (Piece::Yellow, Piece::Yellow) => return true,
                _ => return false,
            }
        }

        // Check inside column
        let col = self.columns[col_idx];
        if row_idx >= 3 {
            if col[row_idx - 3..=row_idx]
                .iter()
                .all(|&piece| compare_pieces(col[row_idx], piece))
            {
                return true;
            }
        }

        let piece = self.columns[col_idx][row_idx];
        // Check horizontal
        let mut left = col_idx;
        let mut right = col_idx;
        while left > 0 {
            if compare_pieces(self.columns[left - 1][row_idx], piece) {
                left -= 1;
            } else {
                break;
            }
        }
        while right < self.columns.len() - 1 {
            if compare_pieces(self.columns[right + 1][row_idx], piece) {
                right += 1;
            } else {
                break;
            }
        }

        if right - left >= 3 {
            return true;
        }

        // Check first diagonal
        let mut top = row_idx;
        let mut right = col_idx;
        while right < self.columns.len() - 1
            && top < col.len() - 1
            && compare_pieces(self.columns[right + 1][top + 1], piece)
        {
            top += 1;
            right += 1;
        }

        let mut bottom = row_idx;
        let mut left = col_idx;
        while left > 0 && bottom > 0 && compare_pieces(self.columns[left - 1][bottom - 1], piece) {
            bottom -= 1;
            left -= 1;
        }

        if right - left >= 3 {
            return true;
        }

        // Check second diagonal
        let mut top = row_idx;
        let mut left = col_idx;
        while left > 0
            && top < col.len() - 1
            && compare_pieces(self.columns[left - 1][top + 1], piece)
        {
            top += 1;
            left -= 1;
        }

        let mut bottom = row_idx;
        let mut right = col_idx;
        while right < self.columns.len() - 1
            && bottom > 0
            && compare_pieces(self.columns[right + 1][bottom - 1], piece)
        {
            bottom -= 1;
            right += 1;
        }

        if right - left >= 3 {
            return true;
        }

        return false;
    }

    pub fn display(&self, piece: Piece) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear screen

        // Rules of the games
        println!("Welcome to Connect 4!");
        println!(
            " - Use the {} and {} arrow keys to navigate",
            "Left".red(),
            "Right".yellow()
        );
        println!(" - Press {} to play", "Enter".green());
        println!(" - Press {} to quit", "q".red());

        // User input
        for i in 0..self.columns.len() {
            print!(" ");
            if i == self.curr {
                match piece {
                    Piece::Red => print!("🔴"),
                    Piece::Yellow => print!("🟡"),
                    _ => {}
                }
            } else {
                print!("  ")
            }
        }
        println!("");
        println!("╠══╬══╬══╬══╬══╬══╬══╣");
        for j in (0..6).rev() {
            for i in 0..self.columns.len() {
                print!("║");
                match self.columns[i][j] {
                    Piece::Red => print!("🔴"),
                    Piece::Yellow => print!("🟡"),
                    _ => print!("  "),
                }
            }
            println!("║")
        }
        println!("╚══╩══╩══╩══╩══╩══╩══╝");
    }

    pub fn go_right(&mut self) {
        if self.curr < 6 {
            self.curr = self.curr + 1;
        }
    }

    pub fn go_left(&mut self) {
        if self.curr > 0 {
            self.curr = self.curr - 1;
        }
    }
}
