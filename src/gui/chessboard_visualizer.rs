use crate::{
    BoardPosition, ChessBoard,
    {gui::TupleWrapper, ChessPiece, Color},
};

use fltk::{app, button::Button, image::PngImage, prelude::*, window::Window};
use ndarray::Array2;
use std::sync::{Arc, RwLock};

pub fn visualize_board(board: ChessBoard) {
    let board = Arc::new(RwLock::new(board));
    let position_from: Arc<RwLock<Option<BoardPosition>>> = Arc::new(RwLock::new(None));
    let needs_redraw = Arc::new(RwLock::new(true));
    let app = app::App::default();

    let win_size = 512;
    let mut wind = Window::default()
        .with_size(win_size, win_size)
        .with_label("Project Smartypants");

    wind.set_icon(PngImage::from_data(include_bytes!("resources/icon.png")).ok());

    let dim = 8;
    let cell_size = win_size / dim;

    // initiate a matrix of buttons
    let mut btn_matrix = Array2::from_shape_fn([dim as usize, dim as usize], |(row, col)| {
        let cloned_board = board.clone();
        let cloned_position_from = position_from.clone();
        let cloned_needs_redraw = needs_redraw.clone();

        let mut but = Button::new(
            col as i32 * cell_size,
            row as i32 * cell_size,
            cell_size,
            cell_size,
            &format!("{}{}", (b'A' + 7 - col as u8) as char, row + 1) as &str,
        );

        but.set_color(if (row + col) % 2 == 0 {
            fltk::enums::Color::White
        } else {
            fltk::enums::Color::Light2
        });

        but.set_label_size(11);
        but.set_frame(fltk::enums::FrameType::FlatBox);

        but.set_callback(move |_but| {
            println!("running callback on {:?}", (row, col));
            if let Ok(mut cloned_position_from) = cloned_position_from.write() {
                if cloned_position_from.is_some_and(|from| {
                    if let Ok(mut board) = cloned_board.write() {
                        board
                            .move_piece(&from, &BoardPosition::from_idx(row as usize, col as usize))
                    } else {
                        false
                    }
                }) {
                    let _ = cloned_needs_redraw
                        .write()
                        .map(|mut writeable| *writeable = true);
                } else {
                    *cloned_position_from =
                        Some(BoardPosition::from_idx(row as usize, col as usize))
                }
            }
        });

        (but, (row, col))
    });

    wind.end();
    wind.show();

    while app.wait() {
        if needs_redraw
            .read()
            .map(|bool_behind_guard| *bool_behind_guard == true)
            .unwrap_or(false)
        {
            println!("redrawing");
            let _ = needs_redraw.write().map(|mut writable| {
                *writable = false;
            });
            btn_matrix.map_mut(|(but, (row, col))| {
                if let Ok(board) = board.read() {
                    but.set_image(
                        board
                            .get_piece_at_position(&BoardPosition::from_idx(*row, *col))
                            .and_then(|piece_color| {
                                TupleWrapper::from(piece_color).into_shared_image()
                            }),
                    );
                    but.redraw();
                }
            });
        }
    }
}
