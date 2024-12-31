pub use connectfour::board::Board;
pub use connectfour::piece::Piece;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrow_key_are_in_bounds() {
        let mut board = Board::new();

        board.go_left();
        assert_eq!(board.curr, 0);

        board.go_right();
        board.go_right();
        board.go_right();
        board.go_right();
        board.go_right();
        board.go_right();

        assert_eq!(board.curr, 6);
    }

    #[test]
    fn column_is_filled() {
        let mut board = Board::new();
        let piece = Piece::Red;
        let col_idx = 0;

        board.add_piece(col_idx, piece);
        board.add_piece(col_idx, piece);
        board.add_piece(col_idx, piece);
        board.add_piece(col_idx, piece);
        board.add_piece(col_idx, piece);
        board.add_piece(col_idx, piece);

        assert_eq!(board.check_col_availability(col_idx), false);
    }

    #[test]
    fn vertical_win() {
        let mut board = Board::new();
        let piece = Piece::Red;

        board.add_piece(board.curr, piece);
        board.add_piece(board.curr, piece);
        board.add_piece(board.curr, piece);
        board.add_piece(board.curr, piece);

        assert_eq!(board.check_win(3, 0), true);
    }

    #[test]
    fn horizontal_win() {
        let mut board = Board::new();
        let piece = Piece::Red;

        board.add_piece(board.curr, piece);
        board.go_right();
        board.add_piece(board.curr, piece);
        board.go_right();
        board.add_piece(board.curr, piece);
        board.go_right();
        board.add_piece(board.curr, piece);

        assert_eq!(board.check_win(0, board.curr), true);
    }

    #[test]
    fn top_diagonal_win() {
        let mut board = Board::new();
        let piece = Piece::Red;

        board.add_piece(board.curr, piece);
        board.go_right();
        board.add_piece(board.curr, piece);
        board.add_piece(board.curr, piece);
        board.go_right();
        board.add_piece(board.curr, piece);
        board.add_piece(board.curr, piece);
        board.add_piece(board.curr, piece);
        board.go_right();
        board.add_piece(board.curr, piece);
        board.add_piece(board.curr, piece);
        board.add_piece(board.curr, piece);
        board.add_piece(board.curr, piece);

        board.display(piece);

        assert_eq!(board.check_win(1, 1), true);
    }

    #[test]
    fn bottom_diagonal_win() {
        let mut board = Board::new();
        let piece = Piece::Red;

        board.add_piece(board.curr, piece);
        board.add_piece(board.curr, piece);
        board.add_piece(board.curr, piece);
        board.add_piece(board.curr, piece);
        board.go_right();
        board.add_piece(board.curr, piece);
        board.add_piece(board.curr, piece);
        board.add_piece(board.curr, piece);
        board.go_right();
        board.add_piece(board.curr, piece);
        board.add_piece(board.curr, piece);
        board.go_right();
        board.add_piece(board.curr, piece);

        board.display(piece);
        assert_eq!(board.check_win(0, board.curr), true);
    }
}
