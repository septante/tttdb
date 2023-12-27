use ttt::state::{Board, Player::*, Space};

#[test]
fn test_import_board() {
    let state = [
        Space::Filled { player: O },
        Space::Filled { player: O },
        Space::Filled { player: O },
        Space::Filled { player: X },
        Space::Filled { player: X },
        Space::Filled { player: X },
        Space::Empty,
        Space::Empty,
        Space::Empty,
    ];
    let board = Board::new(state);
    let board1: Board = "oooxxx---".try_into().unwrap();
    assert!(board1 == board);
}

#[test]
fn test_win_check() {
    let board: Board = "ooo------".try_into().unwrap();
    assert!(board.o_wins());
    let board: Board = "-ooo-----".try_into().unwrap();
    assert!(!board.has_winner());
    let board: Board = "x--x--x--".try_into().unwrap();
    assert!(board.x_wins());
    let board: Board = "o---o---o".try_into().unwrap();
    assert!(board.o_wins());
}
