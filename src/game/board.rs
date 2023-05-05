use std::fmt;

use super::cell::Cell;
use crate::error;

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
        board.reset_hints();
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

    pub fn is_done(&self) -> bool {
        let (hhints, vhints) = self.calculate();
        for x in 0..W {
            if hhints[x] != self.hhints[x] {
                return false;
            }
        }
        for y in 0..H {
            if vhints[y] != self.vhints[y] {
                return false;
            }
        }
        true
    }

    fn check_coordinates(x: usize, y: usize) -> anyhow::Result<()> {
        if x >= W {
            return Err(error!("x [{x}] cannot be greater or equal to {W}").into());
        }
        if y >= H {
            return Err(error!("y [{y}] cannot be greater or equal to {H}").into());
        }
        Ok(())
    }

    fn reset_hints(&mut self) {
        let (hhints, vhints) = self.calculate();
        self.hhints = hhints;
        self.vhints = vhints;
    }

    #[inline]
    fn calculate(&self) -> ([Vec<usize>; W], [Vec<usize>; H]) {
        (self.calculate_hhints(), self.calculate_vhints())
    }

    fn calculate_hhints(&self) -> [Vec<usize>; W] {
        let mut hhints: [Vec<usize>; W] = [(); W].map(|_| Vec::new());
        for x in 0..W {
            let mut last = false;
            let mut count = 0_usize;
            for y in 0..H {
                if !last {
                    if count > 0 {
                        hhints[x].push(count);
                        count = 0;
                    }
                }
                if self.data[y][x].into() {
                    last = true;
                    count = count + 1;
                } else {
                    last = false;
                }
            }
            if count > 0 {
                hhints[x].push(count);
            }
        }
        hhints
    }

    fn calculate_vhints(&self) -> [Vec<usize>; H] {
        let mut vhints: [Vec<usize>; H] = [(); H].map(|_| Vec::new());
        for y in 0..H {
            let mut last = false;
            let mut count = 0_usize;
            for x in 0..W {
                if !last {
                    if count > 0 {
                        vhints[y].push(count);
                        count = 0;
                    }
                }
                if self.data[y][x].into() {
                    last = true;
                    count = count + 1;
                } else {
                    last = false;
                }
            }
            if count > 0 {
                vhints[y].push(count);
            }
        }
        vhints
    }

    #[inline]
    fn pairs() -> impl Iterator<Item = (usize, usize)> {
        (0..H).flat_map(|y| (0..W).map(move |x| (x, y)))
    }
}

impl<const W: usize, const H: usize> Default for Board<W, H> {
    #[inline]
    fn default() -> Self {
        Self {
            hhints: [(); W].map(|_| Vec::new()),
            vhints: [(); H].map(|_| Vec::new()),
            data: [[Cell::default(); W]; H],
        }
    }
}

impl<const W: usize, const H: usize> fmt::Display for Board<W, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        let count = *self.hhints.clone().map(|e| e.len()).iter().max().unwrap();
        for x in 0..W {
            for i in 0..count {
                let cur = self.hhints[x]
                    .get(i)
                    .map(|e| format!("{e:X}"))
                    .unwrap_or(" ".to_owned());
                res = format!("{res}{cur}");
            }
            res = format!("{res}\n");
        }
        for y in 0..H {
            for x in 0..W {
                res = match self.data[y][x] {
                    Cell::Closed => format!("{res}."),
                    Cell::Yes => format!("{res}O"),
                    Cell::No => format!("{res} "),
                }
            }
            for value in self.vhints[y].iter() {
                res = format!("{res}{value:X}");
            }
            res = format!("{res}\n");
        }
        write!(f, "{res}")
    }
}

/*******************************************************************************
 * Tests
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_should_start_all_closed() {
        let board = Board::<3, 2>::default();
        for y in 0..2 {
            for x in 0..3 {
                assert_eq!(board.get(x, y).unwrap(), Cell::Closed);
            }
        }
    }

    #[test]
    fn test_board_get_set() {
        let mut board = Board::<3, 2>::default();
        board.set(0, 0, Cell::Yes).unwrap(); // +---+
        board.set(2, 0, Cell::Yes).unwrap(); // |O O|
        board.set(0, 1, Cell::Yes).unwrap(); // |OO |
        board.set(1, 1, Cell::Yes).unwrap(); // +---+
        for y in 0..2 {
            for x in 0..3 {
                let cell: bool = board.get(x, y).unwrap().into();
                if !cell {
                    board.set(x, y, false.into()).unwrap();
                }
            }
        }
        board.reset_hints();
        assert_eq!(board.hhints[0], vec![2]);
        assert_eq!(board.hhints[1], vec![1]);
        assert_eq!(board.hhints[2], vec![1]);
        assert_eq!(board.vhints[0], vec![1, 1]);
        assert_eq!(board.vhints[1], vec![2]);
    }

    #[test]
    fn it_should_test_done() {
        let mut board = Board::<3, 2>::default();
        board.set(0, 0, Cell::Yes).unwrap(); // +---+
        board.set(2, 0, Cell::Yes).unwrap(); // |O O|
        board.set(0, 1, Cell::Yes).unwrap(); // |OO |
        board.set(1, 1, Cell::Yes).unwrap(); // +---+
        assert!(!board.is_done());
        board.reset_hints();
        assert!(board.is_done());
    }
}
