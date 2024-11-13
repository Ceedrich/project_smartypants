use gui::GameWindow;
use project_smartypants::*;

fn main() {
    let board = ChessBoard::init_default();
    let window = GameWindow::new(board);
    window.start();
}
