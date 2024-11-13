use super::game_state::GameState;
use crate::{BoardPosition, ChessBoard};
use fltk::{app, button::Button, image::PngImage, prelude::*, window::Window};
use ndarray::Array2;
use std::sync::{Arc, RwLock};

pub struct GameWindow {
    app: app::App,
    button_matrix: Array2<(Button, (usize, usize))>,
    window: Window,
    state: Arc<RwLock<GameState>>,
}
impl GameWindow {
    const WIN_SIZE: i32 = 512;
    const DIM: i32 = 8;

    pub fn new(board: ChessBoard) -> Self {
        let state = GameState::new_arc(board);

        let app = app::App::default();

        let mut window = Window::default()
            .with_size(Self::WIN_SIZE, Self::WIN_SIZE)
            .with_label("Project Smartypants");

        window.set_icon(PngImage::from_data(include_bytes!("resources/icon.png")).ok());

        let button_matrix = Self::initialize_button_matrix(state.clone());

        window.end();
        Self {
            app,
            button_matrix,
            window,
            state,
        }
    }
    pub fn start(mut self) {
        self.window.show();

        while self.app.wait() {
            if let Ok(mut state) = self.state.write() {
                state.tick(&mut self.button_matrix);
            }
        }
    }

    fn initialize_button_matrix(
        game_state: Arc<RwLock<GameState>>,
    ) -> Array2<(Button, (usize, usize))> {
        let cell_size = Self::WIN_SIZE / Self::DIM;
        Array2::from_shape_fn([Self::DIM as usize, Self::DIM as usize], |(row, col)| {
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

            let game_state = game_state.clone();
            but.set_callback(move |_but| {
                println!("running callback on {:?}", (row, col));
                if let Ok(mut game_state) = game_state.write() {
                    let clicked_pos = BoardPosition::from_idx(row as usize, col as usize);
                    if game_state.position_from.is_some_and(|from| {
                        game_state
                            .board
                            .move_piece(&from, &BoardPosition::from_idx(row as usize, col as usize))
                    }) {
                        game_state.current_player = !game_state.current_player;
                        game_state.needs_redraw = true;
                    } else if game_state
                        .board
                        .get_piece_at_position(&clicked_pos)
                        .is_some_and(|(_piece, color)| color == game_state.current_player)
                    {
                        game_state.position_from = Some(clicked_pos)
                    }
                }
            });
            (but, (row, col))
        })
    }
}
