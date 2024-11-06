use project_smartypants::*;

fn main() {
    let mut board = ChessBoard::init_default();
    // illegal!
    println!(
        "moved piece: {}",
        board.move_piece(&BoardPosition { x: 0, y: 0 }, &BoardPosition { x: 1, y: 1 })
    );

    println!(
        "moved piece: {}",
        board.move_piece(&BoardPosition { x: 0, y: 1 }, &BoardPosition { x: 0, y: 3 })
    );
}
