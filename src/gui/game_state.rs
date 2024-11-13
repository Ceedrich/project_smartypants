use crate::{
    BoardPosition, ChessBoard,
    {gui::TupleWrapper, Color},
};

use fltk::{button::Button, prelude::*};
use ndarray::Array2;
use std::sync::{Arc, RwLock};

pub struct GameState {
    pub board: ChessBoard,
    pub current_player: Color,
    pub needs_redraw: bool,
    pub position_from: Option<BoardPosition>,
}
impl GameState {
    fn new(board: ChessBoard) -> Self {
        Self {
            board,
            current_player: Color::White,
            needs_redraw: true,
            position_from: None,
        }
    }
    pub fn new_arc(board: ChessBoard) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self::new(board)))
    }

    pub fn tick(&mut self, button_matrix: &mut Array2<(Button, (usize, usize))>) {
        if self.needs_redraw {
            self.needs_redraw = false;
            button_matrix.map_mut(|(but, (row, col))| {
                but.set_image(
                    self.board
                        .get_piece_at_position(&BoardPosition::from_idx(*row, *col))
                        .and_then(|piece_color| {
                            TupleWrapper::from(piece_color).into_shared_image()
                        }),
                );
                but.redraw();
            });
        }
    }
}
