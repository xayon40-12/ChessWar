use chess_referee::Board;

fn main() {
    let mut board = Board::initialise_board();
    loop {
        board.turn();
    }
}
