use super::ChessBoard;
use ndarray::Array2;

#[allow(unused)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ChessPiece {
    Pawn,
    Bishoph,
    Rook,
    Knight,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

#[derive(Clone, Copy, Debug)]
pub struct BoardPosition {
    pub x: u8,
    pub y: u8,
}

impl BoardPosition {
    pub fn forward(&self, color: &Color, count: u8) -> Option<BoardPosition> {
        match color {
            Color::White => self.y.checked_sub(count),
            Color::Black => self.y.checked_add(count),
        }
        .and_then(|new_y| {
            let mut out = *self;
            out.y = new_y;
            out.is_in_bounds().then_some(out)
        })
    }

    pub fn backward(&self, color: &Color, count: u8) -> Option<BoardPosition> {
        match color {
            Color::White => self.y.checked_add(count),
            Color::Black => self.y.checked_sub(count),
        }
        .and_then(|new_y| {
            let mut out = *self;
            out.y = new_y;
            out.is_in_bounds().then_some(out)
        })
    }

    pub fn left(&self, color: &Color, count: u8) -> Option<BoardPosition> {
        match color {
            Color::White => self.x.checked_sub(count),
            Color::Black => self.x.checked_add(count),
        }
        .and_then(|new_x| {
            let mut out = *self;
            out.x = new_x;
            out.is_in_bounds().then_some(out)
        })
    }

    pub fn right(&self, color: &Color, count: u8) -> Option<BoardPosition> {
        match color {
            Color::White => self.x.checked_add(count),
            Color::Black => self.x.checked_sub(count),
        }
        .and_then(|new_x| {
            let mut out = *self;
            out.x = new_x;
            out.is_in_bounds().then_some(out)
        })
    }

    pub fn diag_fr(&self, color: &Color, count: u8) -> Option<BoardPosition> {
        self.forward(color, count)
            .and_then(|forward| forward.right(color, count))
    }

    pub fn diag_fl(&self, color: &Color, count: u8) -> Option<BoardPosition> {
        self.forward(color, count)
            .and_then(|forward| forward.left(color, count))
    }

    pub fn diag_br(&self, color: &Color, count: u8) -> Option<BoardPosition> {
        self.backward(color, count)
            .and_then(|forward| forward.right(color, count))
    }

    pub fn diag_bl(&self, color: &Color, count: u8) -> Option<BoardPosition> {
        self.backward(color, count)
            .and_then(|forward| forward.left(color, count))
    }

    pub fn is_in_bounds(&self) -> bool {
        !(self.x > 7 || self.y > 7)
    }
    pub fn get_idx(&self) -> [usize; 2] {
        [self.y.into(), self.x.into()]
    }
}

pub trait Piece {
    fn get_moves(position: &BoardPosition, board: &ChessBoard) -> Array2<bool>;
    fn get_starting_layout(&self) -> Array2<Option<Color>>;
    fn get_starting_count(&self) -> usize {
        self.get_starting_layout()
            .iter()
            .filter(|&value| value.is_some())
            .count()
    }
}

impl Piece for ChessPiece {
    fn get_starting_layout(&self) -> Array2<Option<Color>> {
        let mut layout = Array2::from_elem((8, 8), None);
        match self {
            Self::King => {
                layout[[0, 4]] = Some(Color::Black);
                layout[[7, 4]] = Some(Color::White);
            }
            Self::Queen => {
                layout[[0, 3]] = Some(Color::Black);
                layout[[7, 3]] = Some(Color::White);
            }
            Self::Bishoph => {
                layout[[0, 2]] = Some(Color::Black);
                layout[[0, 5]] = Some(Color::Black);
                layout[[7, 2]] = Some(Color::White);
                layout[[7, 5]] = Some(Color::White);
            }
            Self::Knight => {
                layout[[0, 1]] = Some(Color::Black);
                layout[[0, 6]] = Some(Color::Black);
                layout[[7, 1]] = Some(Color::White);
                layout[[7, 6]] = Some(Color::White);
            }
            Self::Rook => {
                layout[[0, 0]] = Some(Color::Black);
                layout[[0, 7]] = Some(Color::Black);
                layout[[7, 0]] = Some(Color::White);
                layout[[7, 7]] = Some(Color::White);
            }
            Self::Pawn => {
                layout[[1, 0]] = Some(Color::Black);
                layout[[1, 1]] = Some(Color::Black);
                layout[[1, 2]] = Some(Color::Black);
                layout[[1, 3]] = Some(Color::Black);
                layout[[1, 4]] = Some(Color::Black);
                layout[[1, 5]] = Some(Color::Black);
                layout[[1, 6]] = Some(Color::Black);
                layout[[1, 7]] = Some(Color::Black);

                layout[[6, 0]] = Some(Color::White);
                layout[[6, 1]] = Some(Color::White);
                layout[[6, 2]] = Some(Color::White);
                layout[[6, 3]] = Some(Color::White);
                layout[[6, 4]] = Some(Color::White);
                layout[[6, 5]] = Some(Color::White);
                layout[[6, 6]] = Some(Color::White);
                layout[[6, 7]] = Some(Color::White);
            }
        };
        layout
    }
    fn get_moves(position: &BoardPosition, board: &ChessBoard) -> Array2<bool> {
        let mut moves = Array2::from_elem((8, 8), false);
        fn move_directions(
            moves: &mut Array2<bool>,
            directions: &[fn(&BoardPosition, &Color, u8) -> Option<BoardPosition>],
            position: &BoardPosition,
            board: &&ChessBoard,
            color: &Color,
            limit: bool,
        ) {
            for direction in directions {
                let mut last_position = *position;

                while let Some(next_position) = direction(&last_position, &color, 1) {
                    // we don't want to capture own pieces
                    if board
                        .get_piece_at_position(&next_position)
                        .is_some_and(|(_, color_at_next_pos)| color_at_next_pos == *color)
                    {
                        break;
                    }

                    last_position = next_position;
                    moves[next_position.get_idx()] = true;
                    // we can only capture the first enemy piece, or we can anyways only move one iteration
                    if board.get_piece_at_position(&next_position).is_some() || limit {
                        break;
                    }
                }
            }
        }
        if let Some((piece, color)) = board.get_piece_at_position(&position) {
            match piece {
                ChessPiece::Pawn => {
                    // todo: en passant
                    if let Some(forward) = position.forward(&color, 1) {
                        if board.get_piece_at_position(&forward).is_none() {
                            // we can move forward, it is empty
                            moves[forward.get_idx()] = true;
                        }
                    }
                    if piece.get_starting_layout()[position.get_idx()].is_some() {
                        // the piece is still at its starting position
                        if let Some(two_forward) = position.forward(&color, 2) {
                            // we can move two forward
                            moves[two_forward.get_idx()] = true;
                        }
                    }
                    if let Some(forward_right) = position.diag_fr(&color, 1) {
                        if board
                            .get_piece_at_position(&position)
                            .is_some_and(|(_, other_color)| other_color != color)
                        {
                            // we can capture diagonal right
                            moves[forward_right.get_idx()] = true;
                        }
                    }
                    if let Some(forward_left) = position.diag_fl(&color, 1) {
                        if board
                            .get_piece_at_position(&position)
                            .is_some_and(|(_, other_color)| other_color != color)
                        {
                            // we can capture diagonal left
                            moves[forward_left.get_idx()] = true;
                        }
                    }
                }
                ChessPiece::Knight => {
                    let directions = vec![
                        [
                            BoardPosition::forward,
                            BoardPosition::left,
                            BoardPosition::right,
                        ],
                        [
                            BoardPosition::left,
                            BoardPosition::forward,
                            BoardPosition::backward,
                        ],
                        [
                            BoardPosition::right,
                            BoardPosition::forward,
                            BoardPosition::backward,
                        ],
                        [
                            BoardPosition::backward,
                            BoardPosition::left,
                            BoardPosition::right,
                        ],
                    ];

                    // primary direction is the one we move twice (e.g. forward), then we can go either left or right.
                    // we filter for those out of bounds and then check if it is occupied for the ones left
                    for [primary_direction, secondary_direction1, secondary_direction2] in
                        directions
                    {
                        if let Some(pos_after_primary) = primary_direction(&position, &color, 2) {
                            for position in [
                                secondary_direction1(&pos_after_primary, &color, 1),
                                secondary_direction2(&pos_after_primary, &color, 1),
                            ]
                            .iter()
                            .filter_map(|pos| *pos)
                            {
                                if !board.get_piece_at_position(&position).is_some_and(
                                    |(_, color_at_next_pos)| color_at_next_pos == color,
                                ) {
                                    moves[position.get_idx()] = true;
                                }
                            }
                        }
                    }
                }
                ChessPiece::Bishoph => {
                    let directions = vec![
                        BoardPosition::diag_bl,
                        BoardPosition::diag_br,
                        BoardPosition::diag_fr,
                        BoardPosition::diag_fl,
                    ];

                    move_directions(&mut moves, &directions, &position, &board, &color, false);
                }
                ChessPiece::Rook => {
                    let directions = vec![
                        BoardPosition::forward,
                        BoardPosition::left,
                        BoardPosition::right,
                        BoardPosition::backward,
                    ];

                    move_directions(&mut moves, &directions, &position, &board, &color, false);
                }
                ChessPiece::Queen => {
                    let directions = vec![
                        BoardPosition::forward,
                        BoardPosition::left,
                        BoardPosition::right,
                        BoardPosition::backward,
                        BoardPosition::diag_bl,
                        BoardPosition::diag_br,
                        BoardPosition::diag_fr,
                        BoardPosition::diag_fl,
                    ];
                    move_directions(&mut moves, &directions, &position, &board, &color, false);
                }
                ChessPiece::King => {
                    let directions = vec![
                        BoardPosition::forward,
                        BoardPosition::left,
                        BoardPosition::right,
                        BoardPosition::backward,
                        BoardPosition::diag_bl,
                        BoardPosition::diag_br,
                        BoardPosition::diag_fr,
                        BoardPosition::diag_fl,
                    ];
                    move_directions(&mut moves, &directions, &position, &board, &color, true);
                }
            };
        }

        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;
    use ChessPiece::*;
    use Color::*;
    #[test]
    fn board_inits() {
        let board = ChessBoard::init_default();

        assert_eq!(
            board.fields,
            arr2(&[
                [
                    Some((Rook, Black)),
                    Some((Knight, Black)),
                    Some((Bishoph, Black)),
                    Some((Queen, Black)),
                    Some((King, Black)),
                    Some((Bishoph, Black)),
                    Some((Knight, Black)),
                    Some((Rook, Black)),
                ],
                [
                    Some((Pawn, Black)),
                    Some((Pawn, Black)),
                    Some((Pawn, Black)),
                    Some((Pawn, Black)),
                    Some((Pawn, Black)),
                    Some((Pawn, Black)),
                    Some((Pawn, Black)),
                    Some((Pawn, Black)),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some((Pawn, White)),
                    Some((Pawn, White)),
                    Some((Pawn, White)),
                    Some((Pawn, White)),
                    Some((Pawn, White)),
                    Some((Pawn, White)),
                    Some((Pawn, White)),
                    Some((Pawn, White)),
                ],
                [
                    Some((Rook, White)),
                    Some((Knight, White)),
                    Some((Bishoph, White)),
                    Some((Queen, White)),
                    Some((King, White)),
                    Some((Bishoph, White)),
                    Some((Knight, White)),
                    Some((Rook, White)),
                ],
            ])
        );
    }
    #[test]
    fn pawn_moves_correct() {
        let chess_board = ChessBoard::init_default();
        let position = BoardPosition { x: 0, y: 1 };
        assert_eq!(
            chess_board.get_piece_at_position(&position),
            Some((Pawn, Black))
        );

        let moves = ChessPiece::get_moves(&position, &chess_board);

        assert_eq!(
            moves,
            arr2(&[
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [true, false, false, false, false, false, false, false],
                [true, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false]
            ])
        );
    }

    #[test]
    fn queen_moves_correct() {
        let mut chess_board = ChessBoard::init_default();
        let position = BoardPosition { x: 3, y: 0 };
        assert_eq!(
            chess_board.get_piece_at_position(&position),
            Some((Queen, Black))
        );

        let moves = ChessPiece::get_moves(&position, &chess_board);

        assert_eq!(
            moves,
            arr2(&[
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false]
            ])
        );

        let other_position = BoardPosition { x: 3, y: 3 };
        chess_board.fields[other_position.get_idx()] =
            chess_board.fields[position.get_idx()].take();

        let moves = ChessPiece::get_moves(&other_position, &chess_board);

        assert_eq!(
            moves,
            arr2(&[
                [false, false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false, false],
                [false, false, true, true, true, false, false, false],
                [true, true, true, false, true, true, true, true],
                [false, false, true, true, true, false, false, false],
                [false, true, false, true, false, true, false, false],
                [true, false, false, true, false, false, true, false],
                [false, false, false, false, false, false, false, false]
            ])
        );
    }
}
