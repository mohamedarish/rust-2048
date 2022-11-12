use rust2048::Board;

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
