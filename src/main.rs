use iced::{Application, Settings};
use rust2048::{Board, Move};
use std::io;

fn main() {
    let mut b = Board::new();

    Board::run(Settings::default());

    b.print_board();

    loop {
        println!("Enter your choice (w=up, a=left, s=down, d=right, q=quit): ");

        let mut inp = String::new();

        io::stdin()
            .read_line(&mut inp)
            .expect("Failed to read the input");

        match inp.as_ref() {
            "w\n" => b.make_move(Move::Up),
            "a\n" => b.make_move(Move::Left),
            "s\n" => b.make_move(Move::Down),
            "d\n" => b.make_move(Move::Right),
            "q\n" => break,
            _ => {
                println!("Invalid input");
                continue;
            }
        }

        b.print_board();

        if b.game_end() && b.full_board() {
            println!("Game over, score: {}", b.score);
            break;
        }
    }
}
