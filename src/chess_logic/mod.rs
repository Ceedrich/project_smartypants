mod pieces;

use ndarray::Array2;
pub use pieces::{BoardPosition, ChessPiece, Color, Piece};

#[derive(Clone)]
pub struct ChessBoard {
    pub fields: Array2<Option<(ChessPiece, Color)>>,
}

impl ChessBoard {
    pub fn get_piece_at_position(&self, position: &BoardPosition) -> Option<(ChessPiece, Color)> {
        self.fields[position.get_idx()]
    }
    pub fn get_all_pieces_and_positions(&self) -> Vec<(ChessPiece, Color, BoardPosition)> {
        self.fields
            .indexed_iter()
            .map(|((row, col), opt)| (opt, BoardPosition::from_idx(row, col)))
            .filter_map(|(opt, pos)| opt.map(|(piece, color)| (piece, color, pos)))
            .collect()
    }
    pub fn new() -> Self {
        let empty_field: Option<(ChessPiece, Color)> = None;
        ChessBoard {
            fields: Array2::from_elem([8, 8], empty_field),
        }
    }
    pub fn init_default() -> Self {
        let mut board = Self::new();
        let pieces = vec![
            ChessPiece::Rook,
            ChessPiece::Pawn,
            ChessPiece::Queen,
            ChessPiece::King,
            ChessPiece::Bishoph,
            ChessPiece::Knight,
        ];
        for piece in pieces {
            let piece_layout = ChessPiece::get_starting_layout(&piece)
                .map(|occupying_color| occupying_color.and_then(|color| Some((piece, color))));

            board
                .fields
                .zip_mut_with(&piece_layout, |on_board, in_default_layout| {
                    *on_board = on_board.or(*in_default_layout);
                });
        }
        board
    }
    fn force_move_piece(&mut self, from: &BoardPosition, to: &BoardPosition) {
        if !to.is_in_bounds() {
            panic!("accessing field out of bounds")
        }

        self.fields[to.get_idx()] = self.fields[from.get_idx()].take();
    }

    pub fn move_piece(&mut self, from: &BoardPosition, to: &BoardPosition) -> bool {
        // check if it is in bound
        if !to.is_in_bounds() {
            return false;
        }

        // colors can't be the same
        if self
            .get_piece_at_position(to)
            .zip(self.get_piece_at_position(from))
            .is_some_and(|((_, color_to), (_, color_from))| color_to == color_from)
        {
            return false;
        }

        // move has to be in the allowed set
        if ChessPiece::get_moves(from, self)[to.get_idx()] != true {
            return false;
        }

        // now we are good, move it.
        self.fields[to.get_idx()] = self.fields[from.get_idx()].take();

        return true;
    }
}
