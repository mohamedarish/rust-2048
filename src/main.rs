use iced::{Application, Settings};
// use rust2048::game_loop;
use rust2048::Board;

fn main() {
    Board::run(Settings::default()).expect("Works perfectly");

    // let mut b = Board::new();

    // game_loop(&mut b);
}
