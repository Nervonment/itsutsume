use super::{Board, Eval, Side, HEIGHT, WIDTH};

#[derive(Clone)]
pub struct SimpleEvalBoard {
    board: [[Side; WIDTH]; HEIGHT],
}

impl Default for SimpleEvalBoard {
    fn default() -> Self {
        Self {
            board: [[Side::None; WIDTH]; HEIGHT],
        }
    }
}

impl Board for SimpleEvalBoard {
    fn set_cell(&mut self, r: usize, c: usize, v: Side) {
        self.board[r][c] = v;
    }
    fn get_cell(&self, r: usize, c: usize) -> Side {
        self.board[r][c]
    }
    fn get_board(&self) -> [[Side; WIDTH]; HEIGHT] {
        self.board
    }
}

impl Eval for SimpleEvalBoard {
    fn eval_black(&mut self) -> i64 {
        let mut v = 0;
        for r in 0..HEIGHT {
            for c in 0..WIDTH {
                let h = HEIGHT as i64;
                let w = WIDTH as i64;
                let pv = h / 2 + w / 2 - (r as i64 - h / 2).abs() - (c as i64 - w / 2).abs();
                v += if self.board[r][c] == Side::Black {
                    pv
                } else if self.board[r][c] == Side::White {
                    -pv
                } else {
                    0
                }
            }
        }
        v
    }
}

impl From<[[Side; WIDTH]; HEIGHT]> for SimpleEvalBoard {
    fn from(value: [[Side; WIDTH]; HEIGHT]) -> Self {
        Self { board: value }
    }
}
