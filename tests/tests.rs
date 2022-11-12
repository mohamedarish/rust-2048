use rust2048::{Board, Move};

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
