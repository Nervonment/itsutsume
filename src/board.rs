use std::fmt::Display;

use crate::utils::eq5;

pub mod complicated_eval;
pub mod simple_eval;

pub const WIDTH: usize = 15;
pub const HEIGHT: usize = 15;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Black,
    White,
    None,
}

impl Side {
    pub fn opposite(&self) -> Side {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
            Self::None => Self::None,
        }
    }
}

pub trait Board: Default {
    fn set_cell(&mut self, r: usize, c: usize, v: Side);
    fn get_cell(&self, r: usize, c: usize) -> Side;
    fn is_cell_empty(&self, r: usize, c: usize) -> bool {
        self.get_cell(r, c) == Side::None
    }
    fn winner(&self) -> Side {
        for r in 0..HEIGHT {
            for c in 0..WIDTH - 4 {
                if !self.is_cell_empty(r, c)
                    && eq5(
                        &self.get_cell(r, c),
                        &self.get_cell(r, c + 1),
                        &self.get_cell(r, c + 2),
                        &self.get_cell(r, c + 3),
                        &self.get_cell(r, c + 4),
                    )
                {
                    return self.get_cell(r, c);
                }
            }
        }

        for r in 0..HEIGHT - 4 {
            for c in 0..WIDTH {
                if !self.is_cell_empty(r, c)
                    && eq5(
                        &self.get_cell(r, c),
                        &self.get_cell(r + 1, c),
                        &self.get_cell(r + 2, c),
                        &self.get_cell(r + 3, c),
                        &self.get_cell(r + 4, c),
                    )
                {
                    return self.get_cell(r, c);
                }
            }
        }

        for r in 0..HEIGHT - 4 {
            for c in 0..WIDTH - 4 {
                if !self.is_cell_empty(r + 2, c + 2)
                    && (eq5(
                        &self.get_cell(r, c),
                        &self.get_cell(r + 1, c + 1),
                        &self.get_cell(r + 2, c + 2),
                        &self.get_cell(r + 3, c + 3),
                        &self.get_cell(r + 4, c + 4),
                    ) || eq5(
                        &self.get_cell(r + 4, c),
                        &self.get_cell(r + 3, c + 1),
                        &self.get_cell(r + 2, c + 2),
                        &self.get_cell(r + 1, c + 3),
                        &self.get_cell(r, c + 4),
                    ))
                {
                    return self.get_cell(r + 2, c + 2);
                }
            }
        }

        Side::None
    }
    fn get_board(&self) -> [[Side; WIDTH]; HEIGHT];
}

pub trait Eval: Board {
    fn eval_black(&mut self) -> i64;
    fn eval_white(&mut self) -> i64 {
        -self.eval_black()
    }
}

pub struct BoardDisplay<'a, B: Board>(&'a B);

pub trait ToBoardDisplay: Board {
    fn display(&self) -> BoardDisplay<Self> {
        BoardDisplay(&self)
    }
}

impl<B: Board> ToBoardDisplay for B {}

impl<B: Board> Display for BoardDisplay<'_, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..HEIGHT {
            for c in 0..WIDTH {
                write!(
                    f,
                    "{}",
                    match self.0.get_cell(r, c) {
                        Side::Black => "⚫",
                        Side::White => "⚪",
                        Side::None => "➕",
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
