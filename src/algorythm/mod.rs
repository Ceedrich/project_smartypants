use crate::{BoardPosition, ChessBoard, ChessPiece, Color, Piece};
use ndarray::{Array2, Zip};
use rand::Rng;
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct Model {
    depth: u8,
    heat_maps: [Array2<f64>; 6],
}

impl Model {
    fn new() -> Self {
        Self {
            depth: 4,
            heat_maps: [
                Array2::from_elem([8, 8], 1.0),
                Array2::from_elem([8, 8], 1.0),
                Array2::from_elem([8, 8], 1.0),
                Array2::from_elem([8, 8], 1.0),
                Array2::from_elem([8, 8], 1.0),
                Array2::from_elem([8, 8], 1.0),
            ],
        }
    }

    pub fn randomize_heat_maps(&mut self, mean: f64, std_dev: f64) {
        for heat_map in self.heat_maps.iter_mut() {
            let mut rng = rand::thread_rng();
            let normal = Normal::new(mean, std_dev).unwrap();

            *heat_map = Array2::from_shape_fn([8, 8], |_| normal.sample(&mut rng));
        }
    }

    pub fn mutate_heat_maps(&mut self, std_dev: f64) {
        let mut rng = rand::thread_rng();
        for heat_map in self.heat_maps.iter_mut() {
            for tile in heat_map {
                let normal = Normal::new(*tile, std_dev).unwrap();
                *tile = normal.sample(&mut rng);
            }
        }
    }

    pub fn breed_heat_maps(&self, other: &Self) -> Self {
        let mut child = Model::new();
        let mut rng = rand::thread_rng();
        for heat_map_idx in 0..6 {
            Zip::from(&mut child.heat_maps[heat_map_idx])
                .and(&self.heat_maps[heat_map_idx])
                .and(&other.heat_maps[heat_map_idx])
                .for_each(|child, &own, &other| {
                    if rng.gen::<bool>() {
                        *child = own;
                    } else {
                        *child = other;
                    }
                })
        }

        child
    }

    pub fn get_mut_heat_map_for(&mut self, piece: ChessPiece) -> &mut Array2<f64> {
        let heat_map_idx = match piece {
            ChessPiece::Pawn => 0,
            ChessPiece::Bishoph => 1,
            ChessPiece::Knight => 2,
            ChessPiece::Rook => 3,
            ChessPiece::Queen => 4,
            ChessPiece::King => 5,
        };
        &mut self.heat_maps[heat_map_idx]
    }
    pub fn get_heat_map_for(&self, piece: ChessPiece) -> &Array2<f64> {
        let heat_map_idx = match piece {
            ChessPiece::Pawn => 0,
            ChessPiece::Bishoph => 1,
            ChessPiece::Knight => 2,
            ChessPiece::Rook => 3,
            ChessPiece::Queen => 4,
            ChessPiece::King => 5,
        };
        &self.heat_maps[heat_map_idx]
    }

    /// Grades a board for white, according to the heat map
    /// use negative score for black
    pub fn grade_board(&self, board: &ChessBoard) -> f64 {
        let mut score = 0.0;
        for (piece, color, position) in board.get_all_pieces_and_positions() {
            let mut delta = self.get_heat_map_for(piece)[position.get_idx()];
            if color == Color::Black {
                delta *= -1.0;
            }
            score += delta;
        }

        score
    }

    pub fn grade_moves(
        &self,
        board: ChessBoard,
        own_color: Color,
        depth: u8,
    ) -> Vec<(BoardPosition, BoardPosition, f64)> {
        // let mut scored_moves = Vec::new();
        let data = board.get_all_pieces_and_positions();
        data.par_iter()
            .fold(
                || Vec::new(),
                |mut scored_moves, (_piece, piece_color, from)| {
                    if own_color == *piece_color {
                        for to in ChessPiece::get_moves(&from, &board)
                            .indexed_iter()
                            .filter_map(|((row, col), can_move)| {
                                can_move.then_some(BoardPosition::from_idx(row, col))
                            })
                        {
                            let mut moved_board = board.clone();
                            if moved_board.move_piece(&from, &to) {
                                let score = if depth < self.depth {
                                    -self
                                        .grade_moves(moved_board, !own_color, depth + 1)
                                        .iter()
                                        .map(|(_, _, score)| score)
                                        .sum::<f64>()
                                } else {
                                    self.grade_board(&moved_board)
                                        * match own_color {
                                            Color::Black => -1.0,
                                            Color::White => 1.0,
                                        }
                                };
                                scored_moves.push((from.clone(), to, score))
                            }
                        }
                    }
                    scored_moves
                },
            )
            .reduce(|| Vec::new(), |a, b| [a, b].concat())

        // scored_moves
        // for (_piece, piece_color, from) in data.par_iter() {
        //     if own_color != piece_color {
        //         continue;
        //     }

        //     for to in ChessPiece::get_moves(&from, &board)
        //         .indexed_iter()
        //         .filter_map(|((row, col), can_move)| {
        //             can_move.then_some(BoardPosition::from_idx(row, col))
        //         })
        //     {
        //         let mut moved_board = board.clone();
        //         if moved_board.move_piece(&from, &to) {
        //             let score = if depth < self.depth {
        //                 -self
        //                     .grade_moves(moved_board, !own_color, depth + 1)
        //                     .iter()
        //                     .map(|(_, _, score)| score)
        //                     .sum::<f64>()
        //             } else {
        //                 self.grade_board(&moved_board)
        //                     * match own_color {
        //                         Color::Black => -1.0,
        //                         Color::White => 1.0,
        //                     }
        //             };
        //             scored_moves.push((from, to, score))
        //         }
        //     }
        // }

        // scored_moves
    }
}

// #[derive(Clone, Debug)]
// pub struct AppliedModel {
//     pub color: Color,
//     pub model: Model,
// }

// impl AppliedModel {
//     pub fn new(color: Color, model: Model) -> Self {
//         Self { color, model }
//     }

//     // fn play_against(&self, other: AppliedModel) -> Option<f64> {
//     //     if other.color == self.color {
//     //         return None;
//     //     }

//     // }
// }

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic_scoring() {
        let mut model = Model::new();
        model.get_mut_heat_map_for(ChessPiece::Pawn)[BoardPosition { x: 0, y: 5 }.get_idx()] = 20.0;
        let mut board = ChessBoard::init_default();
        let score = model.grade_board(&board);
        assert_eq!(score, 0.0);
        assert!(board.move_piece(&BoardPosition { x: 0, y: 6 }, &BoardPosition { x: 0, y: 5 }));
        let score = model.grade_board(&board);
        assert_eq!(score, 19.0);
    }

    #[test]
    fn recursive_scoring() {
        let mut model = Model::new();
        model.depth = 1;
        // v1 benchmarks:
        // 5: 5 min
        // 4: 9.18 s
        // 3: 0.35 s
        // 2: 0.02 s
        // 1: 0.00 s

        model.get_mut_heat_map_for(ChessPiece::Pawn)[BoardPosition { x: 0, y: 5 }.get_idx()] = 20.0;
        model.get_mut_heat_map_for(ChessPiece::Pawn)[BoardPosition { x: 0, y: 2 }.get_idx()] = 20.0;

        let board = ChessBoard::init_default();

        let grades = model.grade_moves(board, Color::White, 0);
        dbg!(grades);
    }
}
