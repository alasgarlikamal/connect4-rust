use colored::Colorize;

#[derive(Debug)]
struct Board {
    columns: [[Piece; 6]; 7],
}

#[derive(Copy, Clone, Debug)]
enum Piece {
    Red,
    Yellow,
    Empty,
}

impl Board {
    fn new() -> Board {
        return Board {
            columns: [[Piece::Empty; 6]; 7],
        };
    }

    fn add_piece(&mut self, col_idx: usize, piece: Piece) {
        let row_idx = self.get_available_idx(col_idx);
        let col = self.columns.get_mut(col_idx).unwrap();
        col[row_idx] = piece;
    }

    fn check_col_availability(&self, col_idx: usize) -> bool {
        let col = self.columns[col_idx];
        match col[5] {
            Piece::Empty => return true,
            _ => return false,
        }
    }

    fn get_available_idx(&self, col_idx: usize) -> usize {
        let col = self.columns[col_idx];
        for (row_idx, piece) in col.iter().enumerate() {
            if let Piece::Empty = piece {
                return row_idx;
            }
        }
        unreachable!()
    }

    fn get_available_columns(&self) -> Vec<usize> {
        return (0..self.columns.len())
            .filter(|col_idx| self.check_col_availability(*col_idx))
            .collect();
    }

    fn check_win(&self, row_idx: usize, col_idx: usize) -> bool {
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

    fn display(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear screen
        println!("____________________________________");

        for j in (0..6).rev() {
            for i in 0..self.columns.len() {
                print!("|");
                match self.columns[i][j] {
                    Piece::Red => print!(" 🔴 "),
                    Piece::Yellow => print!(" 🟡 "),
                    _ => print!("    "),
                }
            }
            println!("|")
        }
        println!("------------------------------------");
        println!("[ :0 | :1 | :2 | :3 | :4 | :5 | :6 ]");
    }
}

fn main() {
    println!("Welcome to Connect 4!");
    println!(
        "Enter from {} to {} play your piece.",
        "0".red(),
        "6".yellow()
    );

    let mut board = Board::new();

    let mut turn = true;

    loop {
        board.display();

        let color = if turn { "🔴" } else { "🟡" };
        println!("Enter column number | Turn: {}", color);
        let possible_columns = board.get_available_columns();
        println!("{} {:?}", "Possible columns:".green(), possible_columns);

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let col_idx = input.trim().parse::<usize>();

        let col_idx = match col_idx {
            Ok(val) => val,
            Err(..) => {
                println!("{}", "Value must be a number".red());
                continue;
            }
        };

        if col_idx > 6 {
            println!("Value must be less than {}", "6".yellow());
            continue;
        }

        if !board.check_col_availability(col_idx) {
            println!("Column is not valid!");
            continue;
        }

        let piece = if turn { Piece::Red } else { Piece::Yellow };
        let row_idx = board.get_available_idx(col_idx);
        board.add_piece(col_idx, piece);

        if board.check_win(row_idx, col_idx) {
            board.display();
            println!("{} {} has won!", "Game over!".green(), color);
            break;
        }

        turn = !turn;
    }
}
