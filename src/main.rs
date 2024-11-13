use gui::visualize_board;
use project_smartypants::*;

fn main() {
    let board = ChessBoard::init_default();
    visualize_board(board);
}
