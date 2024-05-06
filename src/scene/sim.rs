use anyhow::bail;
use bevy::ecs::component::Component;
use rand::{
    distributions::{Bernoulli, Distribution},
    thread_rng,
};

/// Instantiates and manages board state in Conway's Game of Life.
#[derive(Debug, Component)]
pub struct ConwayGol {
    board: Vec<Vec<bool>>,
    buffer: Vec<Vec<bool>>,
}

impl ConwayGol {
    /// Returns a Conway GoL board of dim x dim dimensions with a random
    /// initial state
    pub fn build_rand(dim: usize) -> anyhow::Result<Self> {
        if dim < 4 {
            bail!("Board dimension must be greater than 3");
        }

        let mut rng = thread_rng();
        let dist = Bernoulli::new(0.5)?;
        let mut board = Vec::with_capacity(dim);
        for _ in 0..dim {
            board.push(dist.sample_iter(&mut rng).take(dim).collect());
        }

        Ok(Self {
            board,
            buffer: vec![vec![false; dim]; dim],
        })
    }

    #[inline]
    pub fn board(&self) -> &Vec<Vec<bool>> {
        &self.board
    }

    /// Progresses the board to its next state following these rules:
    /// https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
    pub fn tick(&mut self) {
        // the new board is written into the buffer, and then the vectors are swapped
        for row in 0..self.board.len() {
            for col in 0..self.board[0].len() {
                let nb_ct = self.count_neighbors(row, col);
                if !self.board[row][col] {
                    self.buffer[row][col] = nb_ct == 3;
                    continue;
                }
                self.buffer[row][col] = match nb_ct {
                    ct if ct < 2 => false,
                    ct if ct == 2 || ct == 3 => true,
                    ct if ct > 3 => false,
                    _ => panic!("Cases above should have been exhaustive"),
                };
            }
        }
        std::mem::swap(&mut self.board, &mut self.buffer);
    }

    /// Counts the live neighbors a row, col pair has.
    /// Panics if row, col are out of bounds.
    fn count_neighbors(&self, row: usize, col: usize) -> usize {
        assert!(row < self.board.len());
        assert!(col < self.board[0].len());
        let mut ct = 0;

        for nbr in GridIter::new(row, col).into_iter() {
            let Some(is_alive) = self.board.get(nbr.row).and_then(|row| row.get(nbr.col)) else {
                continue;
            };
            if *is_alive {
                ct += 1;
            }
        }

        ct
    }
}

/// Represents a row, column position in a 2d grid
#[derive(Debug, PartialEq, Eq)]
struct Coord {
    row: usize,
    col: usize,
}

/// Iterates over the coordinates adjacent to an origin in a 2d
/// grid.  
#[derive(Debug)]
struct GridIter {
    origin_row: usize,
    origin_col: usize,
    curr_row: usize,
    curr_col: usize,
}

impl GridIter {
    /// Iterates the 2d coordinates around a given row and column. Terminates
    /// in at most 8 iterations. Skips negative indices, but it's the caller's
    /// duty to ensure returned coordinates don't exceed vector bounds.
    pub fn new(origin_row: usize, origin_col: usize) -> Self {
        let curr_row = origin_row.saturating_sub(1);
        let mut curr_col = origin_col.saturating_sub(1);

        if curr_row == origin_row {
            // skip all of the top row if we can't go up
            curr_col = origin_col + 1;
        }

        GridIter {
            origin_row,
            origin_col,
            curr_row,
            curr_col,
        }
    }

    /// Returns the current iterator state as a coordinate.
    fn curr_coord(&self) -> Coord {
        Coord {
            row: self.curr_row,
            col: self.curr_col,
        }
    }
}

impl Iterator for GridIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_row == self.origin_row && self.curr_col == self.origin_col {
            // At origin
            return None;
        }
        if self.curr_row < self.origin_row && self.curr_col <= self.origin_col {
            // top left, top mid
            let res = self.curr_coord();
            self.curr_col += 1;
            return Some(res);
        }
        if self.origin_col < self.curr_col && self.curr_row <= self.origin_row {
            // top right
            // mid right
            let res = self.curr_coord();
            self.curr_row += 1;
            return Some(res);
        }
        if self.origin_col <= self.curr_col && self.origin_row < self.curr_row {
            // bottom right, bottom mid
            let next_col = self.curr_col.saturating_sub(1);
            if next_col < self.curr_col {
                let res = self.curr_coord();
                self.curr_col = next_col;
                return Some(res);
            }
            // bottom mid with a nonexistent left side
            let res = self.curr_coord();
            self.curr_row -= 1;
            debug_assert_eq!(self.curr_row, self.origin_row);
            debug_assert_eq!(self.curr_col, self.origin_col);
            return Some(res);
        }
        if self.origin_row < self.curr_row && self.curr_col < self.origin_col {
            // bottom left
            let res = self.curr_coord();
            self.curr_row -= 1;
            return Some(res);
        }
        debug_assert_eq!(self.curr_row, self.origin_row);
        // left mid
        let res = self.curr_coord();
        self.curr_col += 1;
        debug_assert_eq!(self.curr_col, self.origin_col);
        debug_assert_eq!(self.curr_row, self.origin_row);
        return Some(res);
    }
}

#[cfg(test)]
mod conway_tests {
    use super::{ConwayGol, Coord, GridIter};

    impl Coord {
        pub fn new(row: usize, col: usize) -> Self {
            Coord { row, col }
        }
    }

    #[test]
    fn board_init() -> anyhow::Result<()> {
        let cw = ConwayGol::build_rand(4)?;
        assert_eq!(cw.board.len(), cw.buffer.len());
        assert_eq!(cw.board[0].len(), cw.buffer[0].len());
        assert_eq!(cw.board.len(), cw.board[0].len());
        Ok(())
    }

    /// Tests the grid iterator on a fully in-bounds 9x9 chunk
    #[test]
    fn grid_iter_basic() {
        let mut it = GridIter::new(1, 1).into_iter();
        assert_eq!(it.next().unwrap(), Coord::new(0, 0), "top left");
        assert_eq!(it.next().unwrap(), Coord::new(0, 1), "top mid");
        assert_eq!(it.next().unwrap(), Coord::new(0, 2), "top right");
        assert_eq!(it.next().unwrap(), Coord::new(1, 2), "mid right");
        assert_eq!(it.next().unwrap(), Coord::new(2, 2), "bottom right");
        assert_eq!(it.next().unwrap(), Coord::new(2, 1), "bottom mid");
        assert_eq!(it.next().unwrap(), Coord::new(2, 0), "bottom left");
        assert_eq!(it.next().unwrap(), Coord::new(1, 0), "mid left");
        assert_eq!(it.next(), None, "Done");
    }

    /// Grid iterator on the top left corner
    #[test]
    fn grid_iter_top_left() {
        let mut it = GridIter::new(0, 0).into_iter();
        assert_eq!(it.next().unwrap(), Coord::new(0, 1), "mid right");
        assert_eq!(it.next().unwrap(), Coord::new(1, 1), "bottom right");
        assert_eq!(it.next().unwrap(), Coord::new(1, 0), "bottom mid");
        assert_eq!(it.next(), None, "Done");
    }

    /// Grid iterator on the top row
    #[test]
    fn grid_iter_top() {
        let mut it = GridIter::new(0, 1).into_iter();
        assert_eq!(it.next().unwrap(), Coord::new(0, 2), "mid right");
        assert_eq!(it.next().unwrap(), Coord::new(1, 2), "bottom right");
        assert_eq!(it.next().unwrap(), Coord::new(1, 1), "bottom mid");
        assert_eq!(it.next().unwrap(), Coord::new(1, 0), "bottom left");
        assert_eq!(it.next().unwrap(), Coord::new(0, 0), "left mid");
        assert_eq!(it.next(), None, "Done");
    }

    /// Grid iterator on the bottom left corner
    #[test]
    fn grid_iter_bottom_left() {
        let mut it = GridIter::new(10, 0).into_iter();
        assert_eq!(it.next().unwrap(), Coord::new(9, 0), "top mid");
        assert_eq!(it.next().unwrap(), Coord::new(9, 1), "top right");
        assert_eq!(it.next().unwrap(), Coord::new(10, 1), "mid right");
        assert_eq!(it.next().unwrap(), Coord::new(11, 1), "bottom right");
        assert_eq!(it.next().unwrap(), Coord::new(11, 0), "bottom mid");
        assert_eq!(it.next(), None, "Done");
    }

    /// Verifies correctness of GoL on a 9x9 grid for a few ticks
    #[test]
    fn conway_9x9() {
        let dim = 3;
        let mut cw = ConwayGol {
            board: vec![
                vec![false, true, false],
                vec![false, true, true],
                vec![false, false, true],
            ],
            buffer: vec![vec![false; dim]; dim],
        };

        cw.tick();
        assert_eq!(
            cw.board,
            vec![
                vec![false, true, true],
                vec![false, true, true],
                vec![false, true, true],
            ]
        );

        cw.tick();
        assert_eq!(
            cw.board,
            vec![
                vec![false, true, true],
                vec![true, false, false],
                vec![false, true, true],
            ]
        );

        cw.tick();
        assert_eq!(
            cw.board,
            vec![
                vec![false, true, false],
                vec![true, false, false],
                vec![false, true, false],
            ]
        );
    }
}
