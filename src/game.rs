use crate::board::{Board, Side};

pub struct Game<B: Board> {
    board: B,
    current_player: Side,
}

pub enum DropResult {
    BlackWins,
    WhiteWins,
    Continue,
}

impl<B: Board> Game<B> {
    pub fn new() -> Self {
        Self {
            board: B::default(),
            current_player: Side::Black,
        }
    }

    pub fn drop(&mut self, r: usize, c: usize) -> Result<DropResult, ()> {
        if !self.board.is_cell_empty(r, c) {
            return Err(());
        }
        self.board.set_cell(r, c, self.current_player);
        self.current_player = self.current_player.opposite();
        Ok(match self.board.winner() {
            Side::Black => DropResult::BlackWins,
            Side::White => DropResult::WhiteWins,
            Side::None => DropResult::Continue,
        })
    }

    pub fn get_board(&self) -> &B {
        &self.board
    }
}
