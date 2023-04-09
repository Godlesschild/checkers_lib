use std::collections::HashSet;

use checkers_lib::{Board, BoardBuilder, CheckersMove, Piece};
#[allow(unused_imports)]
use pretty_assertions::{assert_eq, assert_ne};

#[test]
fn test_regular_capture() -> Result<(), checkers_lib::Error> {
    let (board, board_result) = setup_regular_capture();
    let moves = board.all_possible_moves(true);

    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0].captures().len(), 1);

    assert_eq!(board.applied_move(&moves[0])?, board_result);

    Ok(())
}

fn setup_regular_capture() -> (Board, Board) {
    let a = BoardBuilder::try_from_template([
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 2, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ])
    .unwrap()
    .build();
    let b = BoardBuilder::try_from_template([
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ])
    .unwrap()
    .build();

    (a, b)
}

#[test]
fn test_backwards_capture() -> Result<(), checkers_lib::Error> {
    let (board, board_result) = setup_backwards_capture();
    let moves = board.all_possible_moves(true);

    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0].captures().len(), 1);

    assert_eq!(board.applied_move(&moves[0])?, board_result);

    Ok(())
}

fn setup_backwards_capture() -> (Board, Board) {
    let a = BoardBuilder::try_from_template([
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 2, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ])
    .unwrap()
    .build();
    let b = BoardBuilder::try_from_template([
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ])
    .unwrap()
    .build();

    (a, b)
}

#[test]
fn test_multiple_capture() -> Result<(), checkers_lib::Error> {
    let (board, (board_2_result, board_3_result), (capture_2_result, capture_3_result)) =
        setup_multiple_capture();
    let moves = board.all_possible_moves(true);

    assert_eq!(moves.len(), 2);

    let capture_2 = moves.iter().find(|i| i.captures().len() == 2).unwrap();
    let capture_3 = moves.iter().find(|i| i.captures().len() == 3).unwrap();

    assert_eq!(*capture_2, capture_2_result);
    assert_eq!(*capture_3, capture_3_result);

    assert_eq!(board.clone().applied_move(capture_2)?, board_2_result);
    assert_eq!(board.applied_move(capture_3)?, board_3_result);

    Ok(())
}

fn setup_multiple_capture() -> (Board, (Board, Board), (CheckersMove, CheckersMove)) {
    let a = BoardBuilder::try_from_template([
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 2, 0, 2, 0, 2, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 2, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ])
    .unwrap()
    .build();
    let b = BoardBuilder::try_from_template([
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0],
        [0, 2, 0, 2, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ])
    .unwrap()
    .build();
    let c = BoardBuilder::try_from_template([
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 2, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ])
    .unwrap()
    .build();

    let old = Piece::new(false, true, (2, 5).try_into().unwrap());

    let d = CheckersMove::arbitrary(
        old,
        Piece::new(false, true, (6, 1).try_into().unwrap()),
        HashSet::from([(3, 4).try_into().unwrap(), (5, 2).try_into().unwrap()]),
    );

    let e = CheckersMove::arbitrary(
        old,
        Piece::new(false, true, (0, 3).try_into().unwrap()),
        HashSet::from([
            (3, 4).try_into().unwrap(),
            (3, 2).try_into().unwrap(),
            (1, 2).try_into().unwrap(),
        ]),
    );

    (a, (b, c), (d, e))
}

#[test]
fn test_regular_capture_with_promotion() -> Result<(), checkers_lib::Error> {
    let (board, board_result) = setup_regular_capture_with_promotion();

    let moves = board.all_possible_moves(true);

    assert_eq!(moves.len(), 1);
    assert_eq!(moves[0].captures().len(), 1);

    assert_eq!(board.applied_move(&moves[0])?, board_result);

    Ok(())
}

fn setup_regular_capture_with_promotion() -> (Board, Board) {
    let a = BoardBuilder::empty()
        .try_insert(Piece::new(false, true, 10.try_into().unwrap()))
        .unwrap()
        .try_insert(Piece::new(false, false, 7.try_into().unwrap()))
        .unwrap()
        .build();
    let b = BoardBuilder::empty()
        .try_insert(Piece::new(true, true, 3.try_into().unwrap()))
        .unwrap()
        .build();

    (a, b)
}

#[test]
fn test_multiple_capture_with_promotion() -> Result<(), checkers_lib::Error> {
    let (board, boards_result, moves_result) = setup_multiple_capture_with_promotion();
    let moves = board.all_possible_moves(false);

    assert_eq!(moves.len(), 3);

    for capture in moves {
        println!("{capture:?}");
        assert!(moves_result.contains(&capture));

        assert!(boards_result.contains(&board.clone().applied_move(&capture)?));
    }

    Ok(())
}

fn setup_multiple_capture_with_promotion() -> (Board, [Board; 3], [CheckersMove; 3]) {
    let a = BoardBuilder::try_from_template([
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [2, 0, 0, 0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ])
    .unwrap()
    .build();
    let b = BoardBuilder::try_from_template([
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 4, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ])
    .unwrap()
    .build();
    let c = BoardBuilder::try_from_template([
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 4, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ])
    .unwrap()
    .build();
    let d = BoardBuilder::try_from_template([
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 4],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ])
    .unwrap()
    .build();

    let old = Piece::new(false, false, 21.try_into().unwrap());
    let captures = HashSet::from([25.try_into().unwrap(), 23.try_into().unwrap()]);

    let e = CheckersMove::arbitrary(
        old,
        Piece::new(true, false, 19.try_into().unwrap()),
        captures.clone(),
    );

    let f = CheckersMove::arbitrary(
        old,
        Piece::new(true, false, 16.try_into().unwrap()),
        captures.clone(),
    );

    let g = CheckersMove::arbitrary(
        old,
        Piece::new(true, false, 12.try_into().unwrap()),
        captures,
    );

    (a, [b, c, d], [e, f, g])
}
