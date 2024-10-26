use super::{Board, Eval, Side, HEIGHT, WIDTH};

const PADDING: usize = 6;

#[derive(Clone)]
pub struct ComplicatedEvalBoard {
    score_black: i64,
    pub board: [[Side; WIDTH + PADDING * 2]; HEIGHT + PADDING * 2],
}

fn hash(line: [Side; 7]) -> u32 {
    let mut h = 0;
    for v in line {
        h <<= 4;
        h |= match v {
            Side::Black => 0xb,
            Side::White => 0xf,
            Side::None => 0,
        }
    }
    h
}

fn eval_line(line_hash: u32) -> i64 {
    const S7: i64 = 500000000;
    const S6: i64 = 100000;
    const S5: i64 = 30000;
    const S4: i64 = 8000;
    const S3: i64 = 2000;
    const S2: i64 = 300;
    const S1: i64 = 20;
    const S0: i64 = 2;
    let v = match line_hash {
        0x00bbbbb => S7,
        0x0fbbbbb => S7,
        0xf0bbbbb => S7,
        0xffbbbbb => S7,

        0x00bbbb0 => S6,
        0xf0bbbb0 => S6,

        0x00bbbbf => S5,
        0xf0bbbbf => S5,

        0x00b0bbb => S5,
        0x0fb0bbb => S5,
        0xf0b0bbb => S5,
        0xffb0bbb => S5,

        0x00bb0bb => S5,
        0x0fbb0bb => S5,
        0xf0bb0bb => S5,
        0xffbb0bb => S5,

        0x00bbb0b => S5,
        0x0fbbb0b => S5,
        0xf0bbb0b => S5,
        0xffbbb0b => S5,

        0xfbbbb00 => S5,
        0xfbbbb0f => S5,

        0x000bbb0 => S4,
        0xf00bbb0 => S4,

        0x00bbb00 => S4,
        0xf0bbb00 => S4,

        0x00b0bb0 => S3,
        0xf0b0bb0 => S3,

        0x00bb0b0 => S3,
        0xf0bb0b0 => S3,

        0x000bbbf => S2,
        0xf00bbbf => S2,

        0x00b0bbf => S2,
        0xf0b0bbf => S2,

        0x00bb0bf => S2,
        0xf0bb0bf => S2,

        0x0fbbb00 => S2,
        0xffbbb00 => S2,

        0x0fbb0b0 => S2,
        0xffbb0b0 => S2,

        0x0fb0bb0 => S2,
        0xffb0bb0 => S2,

        0x00bb00b => S2,
        0x0fbb00b => S2,
        0xf0bb00b => S2,
        0xffbb00b => S2,

        0x00b00bb => S2,
        0x0fb00bb => S2,
        0xf0b00bb => S2,
        0xffb00bb => S2,

        0x00b0b0b => S2,
        0x0fb0b0b => S2,
        0xf0b0b0b => S2,
        0xffb0b0b => S2,

        0xf0bbb0f => S2,

        0x0000bb0 => S1,
        0xf000bb0 => S1,

        0x000bb00 => S1,
        0xf00bb00 => S1,

        0x00bb000 => S1,
        0xf0bb000 => S1,

        0x000b0b0 => S1,
        0xf00b0b0 => S1,

        0x00b0b00 => S1,
        0xf0b0b00 => S1,

        0x0000bbf => S0,
        0xf000bbf => S0,

        0x000b0bf => S0,
        0xf00b0bf => S0,

        0x00b00bf => S0,
        0xf0b00bf => S0,

        0x00b000b => S0,
        0x0fb000b => S0,
        0xf0b000b => S0,
        0xffb000b => S0,

        0x0fbb000 => S0,
        0xffbb000 => S0,

        0x0fb0b00 => S0,
        0xffb0b00 => S0,

        0x0fb00b0 => S0,
        0xffb00b0 => S0,

        0xf00bb0f => S0,

        0xf0bb00f => S0,

        0xf0b0b0f => S0,

        0x00fffff => -S7,
        0x0bfffff => -S7,
        0xb0fffff => -S7,
        0xbbfffff => -S7,

        0x00ffff0 => -S6,
        0xb0ffff0 => -S6,

        0x00ffffb => -S5,
        0xb0ffffb => -S5,

        0x00f0fff => -S5,
        0x0bf0fff => -S5,
        0xb0f0fff => -S5,
        0xbbf0fff => -S5,

        0x00ff0ff => -S5,
        0x0bff0ff => -S5,
        0xb0ff0ff => -S5,
        0xbbff0ff => -S5,

        0x00fff0f => -S5,
        0x0bfff0f => -S5,
        0xb0fff0f => -S5,
        0xbbfff0f => -S5,

        0xbffff00 => -S5,
        0xbffff0b => -S5,

        0x000fff0 => -S4,
        0xb00fff0 => -S4,

        0x00fff00 => -S4,
        0xb0fff00 => -S4,

        0x00f0ff0 => -S3,
        0xb0f0ff0 => -S3,

        0x00ff0f0 => -S3,
        0xb0ff0f0 => -S3,

        0x000fffb => -S2,
        0xb00fffb => -S2,

        0x00f0ffb => -S2,
        0xb0f0ffb => -S2,

        0x00ff0fb => -S2,
        0xb0ff0fb => -S2,

        0x0bfff00 => -S2,
        0xbbfff00 => -S2,

        0x0bff0f0 => -S2,
        0xbbff0f0 => -S2,

        0x0bf0ff0 => -S2,
        0xbbf0ff0 => -S2,

        0x00ff00f => -S2,
        0x0bff00f => -S2,
        0xb0ff00f => -S2,
        0xbbff00f => -S2,

        0x00f00ff => -S2,
        0x0bf00ff => -S2,
        0xb0f00ff => -S2,
        0xbbf00ff => -S2,

        0x00f0f0f => -S2,
        0x0bf0f0f => -S2,
        0xb0f0f0f => -S2,
        0xbbf0f0f => -S2,

        0xb0fff0b => -S2,

        0x0000ff0 => -S1,
        0xb000ff0 => -S1,

        0x000ff00 => -S1,
        0xb00ff00 => -S1,

        0x00ff000 => -S1,
        0xb0ff000 => -S1,

        0x000f0f0 => -S1,
        0xb00f0f0 => -S1,

        0x00f0f00 => -S1,
        0xb0f0f00 => -S1,

        0x0000ffb => -S0,
        0xb000ffb => -S0,

        0x000f0fb => -S0,
        0xb00f0fb => -S0,

        0x00f00fb => -S0,
        0xb0f00fb => -S0,

        0x00f000f => -S0,
        0x0bf000f => -S0,
        0xb0f000f => -S0,
        0xbbf000f => -S0,

        0x0bff000 => -S0,
        0xbbff000 => -S0,

        0x0bf0f00 => -S0,
        0xbbf0f00 => -S0,

        0x0bf00f0 => -S0,
        0xbbf00f0 => -S0,

        0xb00ff0b => -S0,

        0xb0ff00b => -S0,

        0xb0f0f0b => -S0,

        _ => 0,
    };
    v
}

impl Default for ComplicatedEvalBoard {
    fn default() -> Self {
        Self {
            board: [[Side::None; WIDTH + PADDING * 2]; HEIGHT + PADDING * 2],
            score_black: 0,
        }
    }
}

impl Board for ComplicatedEvalBoard {
    fn set_cell(&mut self, r: usize, c: usize, v: Side) {
        let s1 = self.eval_local(r, c);
        self.board[r + PADDING][c + PADDING] = v;
        let s2 = self.eval_local(r, c);
        self.score_black += s2 - s1;
    }
    fn get_cell(&self, r: usize, c: usize) -> Side {
        self.board[r + PADDING][c + PADDING]
    }
    fn get_board(&self) -> [[Side; WIDTH]; HEIGHT] {
        let mut b = [[Side::None; WIDTH]; HEIGHT];
        for r in 0..HEIGHT {
            for c in 0..WIDTH {
                b[r][c] = self.get_cell(r, c);
            }
        }
        b
    }
}

impl Eval for ComplicatedEvalBoard {
    fn eval_black(&mut self) -> i64 {
        self.score_black
    }

    fn eval_white(&mut self) -> i64 {
        -self.score_black
    }
}

impl ComplicatedEvalBoard {
    fn eval_local(&self, r: usize, c: usize) -> i64 {
        let mut s = 0;

        for r in r as isize - 6..r as isize + 1 {
            let c = c as isize;
            let line = [
                self.get_cell_padding(r, c),
                self.get_cell_padding(r + 1, c),
                self.get_cell_padding(r + 2, c),
                self.get_cell_padding(r + 3, c),
                self.get_cell_padding(r + 4, c),
                self.get_cell_padding(r + 5, c),
                self.get_cell_padding(r + 6, c),
            ];
            s += eval_line(hash(line));
        }

        for c in c as isize - 6..c as isize + 1 {
            let r = r as isize;
            let line = [
                self.get_cell_padding(r, c),
                self.get_cell_padding(r, c + 1),
                self.get_cell_padding(r, c + 2),
                self.get_cell_padding(r, c + 3),
                self.get_cell_padding(r, c + 4),
                self.get_cell_padding(r, c + 5),
                self.get_cell_padding(r, c + 6),
            ];
            s += eval_line(hash(line));
        }

        for i in 0..7 {
            let r = r as isize - 6 + i;
            let c = c as isize - 6 + i;
            let line = [
                self.get_cell_padding(r, c),
                self.get_cell_padding(r + 1, c + 1),
                self.get_cell_padding(r + 2, c + 2),
                self.get_cell_padding(r + 3, c + 3),
                self.get_cell_padding(r + 4, c + 4),
                self.get_cell_padding(r + 5, c + 5),
                self.get_cell_padding(r + 6, c + 6),
            ];
            s += eval_line(hash(line));
        }

        for i in 0..7 {
            let r = r as isize - 6 + i;
            let c = c as isize + 6 - i;
            let line = [
                    self.get_cell_padding(r, c),
                    self.get_cell_padding(r + 1, c - 1),
                    self.get_cell_padding(r + 2, c - 2),
                    self.get_cell_padding(r + 3, c - 3),
                    self.get_cell_padding(r + 4, c - 4),
                    self.get_cell_padding(r + 5, c - 5),
                    self.get_cell_padding(r + 6, c - 6),
                ];
                s += eval_line(hash(line));
        }

        s
    }

    #[inline]
    fn get_cell_padding(&self, r: isize, c: isize) -> Side {
        self.board[(r + PADDING as isize) as usize][(c + PADDING as isize) as usize]
    }
}
