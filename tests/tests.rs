use rust2048::{Board, Move, Tile};

#[test]
fn random_number_test() {
    let new_board = Board::new();

    let mut max = -1;

    for i in new_board.tiles {
        if i.num > max {
            max = i.num;
        }
    }

    assert!(max > 1)
}

#[test]
fn up_movement_test() {
    let mut new_board = Board::new();

    new_board.make_move(Move::Up);

    let mut max = -1;

    for i in 0..4 {
        if new_board.tiles[i].num > max {
            max = new_board.tiles[i].num;
        }
    }

    assert!(max > 1)
}

#[test]
fn down_movement_test() {
    let mut new_board = Board::new();

    new_board.make_move(Move::Down);

    let mut max = -1;

    for i in 0..4 {
        if new_board.tiles[12 + i].num > max {
            max = new_board.tiles[12 + i].num;
        }
    }

    assert!(max > 1)
}

#[test]
fn left_movement_test() {
    let mut new_board = Board::new();

    new_board.make_move(Move::Left);

    let mut max = -1;

    for i in 0..4 {
        if new_board.tiles[i * 4].num > max {
            max = new_board.tiles[i * 4].num;
        }
    }

    assert!(max > 1)
}

#[test]
fn right_movement_test() {
    let mut new_board = Board::new();

    new_board.make_move(Move::Right);

    let mut max = -1;

    for i in 0..4 {
        if new_board.tiles[(i * 4) + 3].num > max {
            max = new_board.tiles[(i * 4) + 3].num;
        }
    }

    new_board.print_board();

    assert!(max > 1)
}

#[test]
fn functionality_test() {
    let new_board = Board {
        tiles: [
            Tile { x: 0, y: 0, num: 2 },
            Tile { x: 1, y: 0, num: 2 },
            Tile { x: 2, y: 0, num: 4 },
            Tile { x: 3, y: 0, num: 4 },
            Tile { x: 0, y: 1, num: 2 },
            Tile { x: 1, y: 1, num: 2 },
            Tile { x: 2, y: 1, num: 4 },
            Tile { x: 3, y: 1, num: 4 },
            Tile { x: 0, y: 2, num: 8 },
            Tile { x: 1, y: 2, num: 8 },
            Tile {
                x: 2,
                y: 2,
                num: 16,
            },
            Tile {
                x: 3,
                y: 2,
                num: 16,
            },
            Tile { x: 0, y: 3, num: 8 },
            Tile { x: 1, y: 3, num: 8 },
            Tile {
                x: 2,
                y: 3,
                num: 16,
            },
            Tile {
                x: 3,
                y: 3,
                num: 16,
            },
        ],
        score: 0,
    };

    let mut lu_test = new_board.clone();
    let mut ru_test = new_board.clone();
    let mut ld_test = new_board.clone();
    let mut rd_test = new_board.clone();

    lu_test.make_move(Move::Left);
    lu_test.make_move(Move::Up);

    ru_test.make_move(Move::Right);
    ru_test.make_move(Move::Up);

    ld_test.make_move(Move::Left);
    ld_test.make_move(Move::Down);

    rd_test.make_move(Move::Right);
    rd_test.make_move(Move::Down);

    let mut lu = false;
    let mut ru = false;
    let mut ld = false;
    let mut rd = false;

    if lu_test.tiles[5].num == 64
        && lu_test.tiles[4].num == 32
        && lu_test.tiles[1].num == 16
        && lu_test.tiles[0].num == 8
    {
        lu = true;
    }

    if ru_test.tiles[7].num == 64
        && ru_test.tiles[6].num == 32
        && ru_test.tiles[3].num == 16
        && ru_test.tiles[2].num == 8
    {
        ru = true;
    }

    if ld_test.tiles[13].num == 64
        && ld_test.tiles[12].num == 32
        && ld_test.tiles[9].num == 16
        && ld_test.tiles[8].num == 8
    {
        ld = true;
    }

    if rd_test.tiles[15].num == 64
        && rd_test.tiles[14].num == 32
        && rd_test.tiles[11].num == 16
        && rd_test.tiles[10].num == 8
    {
        rd = true;
    }

    assert_eq!((lu, ru, ld, rd), (true, true, true, true))
}
