use rust2048::{Board, Move};

fn main() {
    let mut b = Board::new();

    b.print_board();

    b.make_move(Move::Up);
}
