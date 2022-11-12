use std::vec;

use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub tiles: Vec<Tile>,
    score: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    x: i32,
    y: i32,
    pub num: i32,
}

pub enum Move {
    Left,
    Right,
    Up,
    Down,
}

fn gen_tile_number() -> i32 {
    let mut rng = rand::thread_rng();

    2 + 2 * rng.gen_range(0..2)
}

impl Board {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let rand_start = rng.gen_range(0..16);
        let mut tile_place: Vec<Tile> = Vec::new();

        for i in 0..16 {
            let mut current_tile = Tile {
                x: i % 4,
                y: (i - i % 4) / 4,
                num: 0,
            };
            if i == rand_start {
                let new_tile_number = gen_tile_number();
                current_tile = Tile {
                    x: i % 4,
                    y: (i - i % 4) / 4,
                    num: new_tile_number,
                };
            }

            tile_place.append(&mut vec![current_tile]);
        }

        Board {
            tiles: tile_place,
            score: 0,
        }
    }

    pub fn print_board(self) {
        for i in 0..4 {
            print!("|\t");

            for j in 0..4 {
                // println!("{} {}", i, j);
                if self.tiles[4 * i + j].num == 0 {
                    print!(" \t|\t");
                } else {
                    print!("{}\t|\t", self.tiles[4 * i + j].num);
                }
            }

            println!();
        }
    }

    pub fn make_move(&mut self, m: Move) {
        match m {
            Move::Up => {
                let mut indexes: Vec<Vec<usize>> = vec![vec![], vec![], vec![], vec![]];

                for i in 0..16 {
                    if self.tiles[i].num > 0 {
                        indexes[i % 4].append(&mut vec![i]);
                    }
                }

                for i in indexes {
                    for j in &i {
                        self.tiles.swap(*j, 4 - &i.len());
                    }
                }
            }
            Move::Left => {}
            Move::Down => {}
            Move::Right => {}
        }
    }
}
