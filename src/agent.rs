use std::i64;

use rand::random;

use crate::board::{Board, Eval, Side, HEIGHT, WIDTH};

pub trait Agent<B: Board> {
    fn action(&mut self, board: &B, side: Side) -> (usize, usize);
}

pub struct MinimaxSearch {
    max_depth: u32,
    action: (usize, usize),
    side: Side,
}

impl MinimaxSearch {
    pub fn with_max_depth(max_depth: u32) -> Self {
        Self {
            max_depth,
            action: (HEIGHT / 2, WIDTH / 2),
            side: Side::None,
        }
    }
}

impl MinimaxSearch {
    pub fn has_pieces_nearby<B: Board>(state: &mut B, r: usize, c: usize) -> bool {
        for dr in -1..2 {
            for dc in -1..2 {
                let r = r as isize + dr;
                let c = c as isize + dc;
                if r >= 0
                    && r < HEIGHT as isize
                    && c >= 0
                    && c < WIDTH as isize
                    && !state.is_cell_empty(r as usize, c as usize)
                {
                    return true;
                }
            }
        }
        false
    }

    fn max_search<B: Eval + Clone>(
        &mut self,
        state: &mut B,
        mut alpha: i64,
        beta: i64,
        depth: u32,
    ) -> i64 {
        let winner = state.winner();
        if winner == self.side {
            return i64::MAX - depth as i64;
        } else if winner != Side::None {
            return i64::MIN + depth as i64;
        }

        if depth >= self.max_depth {
            return match self.side {
                Side::Black => state.eval_black(),
                Side::White => state.eval_white(),
                Side::None => unreachable!(),
            };
        }

        let mut v = i64::MIN;
        for r in 0..HEIGHT {
            for c in 0..WIDTH {
                if !state.is_cell_empty(r, c) || !MinimaxSearch::has_pieces_nearby(state, r, c) {
                    continue;
                }

                state.set_cell(r, c, self.side);
                let v1 = self.min_search(state, alpha, beta, depth + 1);
                state.set_cell(r, c, Side::None);

                if v1 > v || (v1 == v && random::<bool>()) {
                    v = v1;
                    if depth == 0 {
                        self.action = (r, c);
                    }
                }
                alpha = i64::max(alpha, v);
                if v >= beta {
                    return v;
                }
            }
        }
        v
    }

    fn min_search<B: Eval + Clone>(
        &mut self,
        state: &mut B,
        alpha: i64,
        mut beta: i64,
        depth: u32,
    ) -> i64 {
        let winner = state.winner();
        if winner == self.side {
            return i64::MAX - depth as i64;
        } else if winner != Side::None {
            return i64::MIN + depth as i64;
        }

        if depth >= self.max_depth {
            return match self.side {
                Side::Black => state.eval_black(),
                Side::White => state.eval_white(),
                Side::None => unreachable!(),
            };
        }

        let mut v = i64::MAX;
        for r in 0..HEIGHT {
            for c in 0..WIDTH {
                if !state.is_cell_empty(r, c) || !MinimaxSearch::has_pieces_nearby(state, r, c) {
                    continue;
                }

                state.set_cell(r, c, self.side.opposite());
                let v1 = self.max_search(state, alpha, beta, depth + 1);
                state.set_cell(r, c, Side::None);

                v = i64::min(v, v1);
                beta = i64::min(beta, v);
                if v <= alpha {
                    return v;
                }
            }
        }
        v
    }
}

impl<B: Eval + Clone> Agent<B> for MinimaxSearch {
    fn action(&mut self, board: &B, side: Side) -> (usize, usize) {
        self.side = side;
        self.max_search(&mut board.clone(), i64::MIN, i64::MAX, 0);
        self.action
    }
}
