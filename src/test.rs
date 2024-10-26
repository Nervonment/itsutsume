use crate::{
    agent::{Agent, MinimaxSearch},
    board::{complicated_eval::ComplicatedEvalBoard, Board, Side},
};

#[test]
fn test_eval() {
    for d in 1..5 {
        let mut agent = MinimaxSearch::with_max_depth(d);
        let mut board = ComplicatedEvalBoard::default();
        board.set_cell(5, 5, Side::Black);
        board.set_cell(5, 6, Side::Black);
        board.set_cell(5, 7, Side::Black);
        board.set_cell(5, 8, Side::Black);
        let pos = agent.action(&board, Side::Black);
        assert!(pos == (5, 4) || pos == (5, 9), "pos: {:?} \nd = {}", pos, d);

        let mut agent = MinimaxSearch::with_max_depth(d);
        let mut board = ComplicatedEvalBoard::default();
        board.set_cell(5, 5, Side::Black);
        board.set_cell(6, 5, Side::Black);
        board.set_cell(7, 5, Side::Black);
        board.set_cell(8, 5, Side::Black);
        let pos = agent.action(&board, Side::Black);
        assert!(pos == (4, 5) || pos == (9, 5), "pos: {:?} \nd = {}", pos, d);

        let mut agent = MinimaxSearch::with_max_depth(d);
        let mut board = ComplicatedEvalBoard::default();
        board.set_cell(5, 5, Side::Black);
        board.set_cell(6, 6, Side::Black);
        board.set_cell(7, 7, Side::Black);
        board.set_cell(8, 8, Side::Black);
        let pos = agent.action(&board, Side::Black);
        assert!(pos == (4, 4) || pos == (9, 9), "pos: {:?} \nd = {}", pos, d);

        let mut agent = MinimaxSearch::with_max_depth(d);
        let mut board = ComplicatedEvalBoard::default();
        board.set_cell(5, 8, Side::Black);
        board.set_cell(6, 7, Side::Black);
        board.set_cell(7, 6, Side::Black);
        board.set_cell(8, 5, Side::Black);
        let pos = agent.action(&board, Side::Black);
        assert!(pos == (4, 9) || pos == (9, 4), "pos: {:?} \nd = {}", pos, d);
    }
}
