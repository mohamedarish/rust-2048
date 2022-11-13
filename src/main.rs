use rust2048::{Board, Move};

fn main() {
    let mut b = Board::new();

    b.print_board();

    b.make_move(Move::Left);

    println!();

    b.make_move(Move::Up);

    println!();

    b.print_board();

    b.make_move(Move::Right);

    println!();

    b.print_board();

    b.print_board();

    b.make_move(Move::Down);

    println!();

    b.print_board();
}
