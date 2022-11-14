use iced::{Command, Sandbox, Theme};
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub num: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Board {
    pub tiles: [Tile; 16],
    pub score: i32,
}

#[derive(Debug)]
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
        let mut tile_array = [Tile { x: 0, y: 0, num: 0 }; 16];

        for i in 0..16 {
            let current_tile = Tile {
                x: i % 4,
                y: i / 4,
                num: 0,
            };

            tile_array[i as usize] = current_tile;
        }

        let mut res = Board {
            tiles: tile_array,
            score: 0,
        };

        res.gen_new_number();

        res
    }

    fn gen_new_number(&mut self) {
        let mut rand_pos: i32;
        loop {
            let mut rng = rand::thread_rng();
            rand_pos = rng.gen_range(0..16) as i32;

            if self.tiles[rand_pos as usize].num < 1 {
                break;
            }
        }
        let new_tile_number = gen_tile_number();
        let current_tile = Tile {
            x: rand_pos % 4,
            y: (rand_pos - rand_pos % 4) / 4,
            num: new_tile_number,
        };

        self.tiles[rand_pos as usize] = current_tile;
    }

    fn merge(&mut self, m: Move) -> i32 {
        let mut score = 0;
        match m {
            Move::Up => {
                for i in 0..4 {
                    let mut index = 1;
                    loop {
                        if self.tiles[index * 4 + i].num == 0 {
                            index += 1;
                        } else if self.tiles[index * 4 + i].num
                            == self.tiles[(index + 1) * 4 + i].num
                        {
                            self.tiles[index * 4 + i].num *= 2;
                            score += self.tiles[index * 4 + i].num;

                            self.tiles[(index + 1) * 4 + i].num = 0;
                            for j in (index + 1)..3 {
                                self.tiles[j * 4 + i].num = self.tiles[(j + 1) * 4 + i].num;
                                self.tiles[(j + 1) * 4 + i].num = 0;
                            }
                        } else if self.tiles[(index - 1) * 4 + i].num
                            == self.tiles[index * 4 + i].num
                        {
                            self.tiles[(index - 1) * 4 + i].num *= 2;
                            score += self.tiles[(index - 1) * 4 + i].num;

                            self.tiles[index * 4 + i].num = 0;
                            for j in index..3 {
                                self.tiles[j * 4 + i].num = self.tiles[(j + 1) * 4 + i].num;
                                self.tiles[(j + 1) * 4 + i].num = 0;
                            }
                        } else {
                            index += 1;
                        }

                        if index >= 3 {
                            break;
                        }
                    }
                }
            }
            Move::Left => {
                for i in 0..4 {
                    let mut index = 1;
                    loop {
                        if self.tiles[i * 4 + index].num == 0 {
                            index += 1
                        } else if self.tiles[i * 4 + index].num == self.tiles[i * 4 + index + 1].num
                        {
                            self.tiles[i * 4 + index].num *= 2;
                            score += self.tiles[i * 4 + index].num;

                            self.tiles[i * 4 + index + 1].num = 0;
                            for j in (index + 1)..3 {
                                self.tiles[i * 4 + j].num = self.tiles[i * 4 + j + 1].num;
                                self.tiles[i * 4 + j + 1].num = 0;
                            }
                        } else if self.tiles[i * 4 + index - 1].num == self.tiles[i * 4 + index].num
                        {
                            self.tiles[i * 4 + index - 1].num *= 2;
                            score += self.tiles[i * 4 + index - 1].num;

                            self.tiles[i * 4 + index].num = 0;
                            for j in index..3 {
                                self.tiles[i * 4 + j].num = self.tiles[i * 4 + j + 1].num;
                                self.tiles[i * 4 + j + 1].num = 0;
                            }
                        } else {
                            index += 1;
                        }

                        if index >= 3 {
                            break;
                        }
                    }
                }
            }
            Move::Down => {
                for i in 0..4 {
                    let mut index = 2;
                    loop {
                        if self.tiles[index * 4 + i].num == 0 {
                            index -= 1;
                        } else if self.tiles[index * 4 + i].num
                            == self.tiles[(index + 1) * 4 + i].num
                        {
                            self.tiles[(index + 1) * 4 + i].num *= 2;
                            score += self.tiles[(index + 1) * 4 + i].num;

                            self.tiles[index * 4 + i].num = 0;
                            for j in (0..index).rev() {
                                self.tiles[(j + 1) * 4 + i].num = self.tiles[j * 4 + i].num;
                                self.tiles[j * 4 + i].num = 0;
                            }
                        } else if self.tiles[index * 4 + i].num
                            == self.tiles[(index - 1) * 4 + i].num
                        {
                            self.tiles[index * 4 + i].num *= 2;
                            score += self.tiles[index * 4 + i].num;

                            self.tiles[(index - 1) * 4 + i].num = 0;
                            for j in (0..(index - 1)).rev() {
                                self.tiles[(j + 1) * 4 + i].num = self.tiles[j * 4 + i].num;
                                self.tiles[j * 4 + i].num = 0;
                            }
                        } else {
                            index -= 1;
                        }

                        if index <= 0 {
                            break;
                        }
                    }
                }
            }
            Move::Right => {
                for i in 0..4 {
                    let mut index = 2;
                    loop {
                        if self.tiles[i * 4 + index].num == 0 {
                            index -= 1;
                        } else if self.tiles[i * 4 + index].num == self.tiles[i * 4 + index + 1].num
                        {
                            self.tiles[i * 4 + index + 1].num *= 2;
                            score += self.tiles[i * 4 + index + 1].num;

                            self.tiles[i * 4 + index].num = 0;
                            for j in (0..index).rev() {
                                self.tiles[i * 4 + j + 1].num = self.tiles[i * 4 + j].num;
                                self.tiles[i * 4 + j].num = 0;
                            }
                        } else if self.tiles[i * 4 + index].num == self.tiles[i * 4 + index - 1].num
                        {
                            self.tiles[i * 4 + index].num *= 2;
                            score += self.tiles[i * 4 + index].num;

                            self.tiles[i * 4 + index - 1].num = 0;
                            for j in (0..(index - 1)).rev() {
                                self.tiles[i * 4 + j + 1].num = self.tiles[i * 4 + j].num;
                                self.tiles[i * 4 + j].num = 0;
                            }
                        } else {
                            index -= 1;
                        }

                        if index <= 0 {
                            break;
                        }
                    }
                }
            }
        }

        score
    }

    pub fn game_end(self) -> bool {
        let mut left_clone = self.clone();

        left_clone.make_move(Move::Left);

        let mut right_clone = self.clone();

        right_clone.make_move(Move::Right);

        let mut up_clone = self.clone();

        up_clone.make_move(Move::Up);

        let mut down_clone = self.clone();

        down_clone.make_move(Move::Down);

        if left_clone.score == self.score
            && right_clone.score == self.score
            && up_clone.score == self.score
            && down_clone.score == self.score
        {
            return true;
        }

        false
    }

    pub fn full_board(self) -> bool {
        for i in self.tiles {
            if i.num == 0 {
                return false;
            }
        }

        true
    }

    pub fn print_board(self) {
        println!("\n\nscore: {}\n", self.score);
        for i in 0..4 {
            print!("|\t");

            for j in 0..4 {
                if self.tiles[4 * i + j].num == 0 {
                    print!(" \t|\t");
                } else {
                    print!("{}\t|\t", self.tiles[4 * i + j].num);
                }
            }

            println!();
        }
        println!();
    }

    pub fn make_move(&mut self, m: Move) {
        match m {
            Move::Up => {
                let mut indexes = [[0; 4]; 4];
                let mut ind = [0; 4];
                for i in 0..16 {
                    if self.tiles[i].num > 0 {
                        indexes[i % 4][ind[i % 4]] = i + 1;
                        ind[i % 4] += 1;
                    }
                }

                for i in 0..indexes.len() {
                    let mut index = 0;
                    for j in indexes[i] {
                        if j != 0 {
                            let old_num = self.tiles[index * 4 + i].num;
                            self.tiles[index * 4 + i].num = self.tiles[j - 1].num;
                            self.tiles[j - 1].num = old_num;
                            index += 1;
                        }
                    }
                }
            }
            Move::Left => {
                let mut indexes = [[0; 4]; 4];
                let mut ind = [0; 4];

                for i in 0..16 {
                    if self.tiles[i].num > 0 {
                        indexes[i / 4][ind[i / 4]] = i + 1;
                        ind[i / 4] += 1;
                    }
                }

                for i in 0..indexes.len() {
                    let mut index = 0;
                    for j in indexes[i] {
                        if j != 0 {
                            let old_num = self.tiles[i * 4 + index].num;
                            self.tiles[i * 4 + index].num = self.tiles[j - 1].num;
                            self.tiles[j - 1].num = old_num;
                            index += 1;
                        }
                    }
                }
            }
            Move::Down => {
                let mut indexes = [[0; 4]; 4];
                let mut ind = [0; 4];
                for i in 0..16 {
                    if self.tiles[i].num > 0 {
                        indexes[i % 4][ind[i % 4]] = i + 1;
                        ind[i % 4] += 1;
                    }
                }

                for i in 0..indexes.len() {
                    indexes[i].sort_by(|a, b| b.cmp(a));
                }

                for i in 0..indexes.len() {
                    let mut index = 3;
                    for j in indexes[i] {
                        if j != 0 {
                            let old_num = self.tiles[index * 4 + i].num;
                            self.tiles[index * 4 + i].num = self.tiles[j - 1].num;
                            self.tiles[j - 1].num = old_num;
                            if index > 0 {
                                index -= 1;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
            Move::Right => {
                let mut indexes = [[0; 4]; 4];
                let mut ind = [0; 4];

                for i in 0..16 {
                    if self.tiles[i].num > 0 {
                        indexes[(i - i % 4) / 4][ind[i / 4]] = i + 1;
                        ind[i / 4] += 1;
                    }
                }

                for i in 0..indexes.len() {
                    indexes[i].sort_by(|a, b| b.cmp(a));
                }

                for i in 0..indexes.len() {
                    let mut index = 3;
                    for j in indexes[i] {
                        if j != 0 {
                            let old_num = self.tiles[i * 4 + index].num;
                            self.tiles[i * 4 + index].num = self.tiles[j - 1].num;
                            self.tiles[j - 1].num = old_num;
                            if index > 0 {
                                index -= 1;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }

        self.score += self.merge(m);

        if !self.full_board() {
            self.gen_new_number();
        }
    }
}

impl Sandbox for Board {
    type Message = Move;

    fn new() -> Self {
        Board::new()
    }

    fn title(&self) -> String {
        String::from("2048")
    }

    fn update(&mut self, message: Self::Message) {
        self.make_move(message);
    }

    fn view(&self) -> iced::Element<Self::Message> {
        todo!()
    }
}
