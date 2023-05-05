use super::cell::Cell;
use crate::error::Error;

#[derive(Debug)]
pub struct Board<const W: usize, const H: usize> {
    pub hhints: [Vec<usize>; W],
    pub vhints: [Vec<usize>; H],
    data: [[Cell; W]; H],
}

impl<const W: usize, const H: usize> Board<W, H> {
    pub fn random(with_hints: bool) -> Self {
        let mut board = Self::default();
        for (x, y) in Self::pairs() {
            board.data[y][x] = (fastrand::u8(0..16u8) < 6u8).into();
        }
        board.calculate();
        if with_hints {
            for (x, y) in Self::pairs() {
                if fastrand::u8(0..16u8) < 10u8 {
                    board.data[y][x] = Cell::Closed;
                }
            }
        } else {
            board.data = [[Cell::default(); W]; H];
        }
        board
    }

    pub fn get(&self, x: usize, y: usize) -> anyhow::Result<Cell> {
        Self::check_coordinates(x, y)?;
        Ok(self.data[y][x])
    }

    pub fn set(&mut self, x: usize, y: usize, value: Cell) -> anyhow::Result<()> {
        Self::check_coordinates(x, y)?;
        self.data[y][x] = value;
        Ok(())
    }

    pub fn check(&self) -> bool {
        todo!() // TODO: check whether board matches hints
    }

    fn check_coordinates(x: usize, y: usize) -> anyhow::Result<()> {
        if x >= W {
            return Err(Error(format!("x [{x}] cannot be greater or equal to {W}")).into());
        }
        if y >= H {
            return Err(Error(format!("y [{y}] cannot be greater or equal to {H}")).into());
        }
        Ok(())
    }

    fn calculate(&mut self) {
        todo!() // TODO: calculate hints
    }

    fn pairs() -> impl Iterator<Item = (usize, usize)> {
        (0..H).flat_map(|y| (0..W).map(move |x| (x, y)))
    }
}

impl<const W: usize, const H: usize> Default for Board<W, H> {
    fn default() -> Self {
        Self {
            hhints: [(); W].map(|_| Vec::new()),
            vhints: [(); H].map(|_| Vec::new()),
            data: [[Cell::default(); W]; H],
        }
    }
}
