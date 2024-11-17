use crate::i8vec2::I8Vec2;
use std::fmt::{Display, Formatter};

pub const GRID_SIZE: usize = 8;

/// The hidden inner secret of the game
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct AtomGrid {
    atoms: [[bool; GRID_SIZE]; GRID_SIZE],
}

impl AtomGrid {
    pub fn get(&self, v: I8Vec2) -> bool {
        v.in_grid() && self.atoms[v.x as usize][v.y as usize]
    }
    pub fn set(&mut self, v: I8Vec2, value: bool) {
        if v.in_grid() {
            self.atoms[v.x as usize][v.y as usize] = value;
        } else {
            panic!("Out of bounds. Writing {} to {:?}", value, v);
        }
    }

    pub fn random(atom_count: u8) -> Self {
        let mut this = Self::default();
        let mut placed_down = 0;
        while placed_down < atom_count {
            let v = I8Vec2::random();
            if !this.get(v) {
                this.set(v, true);
                placed_down += 1;
            }
        }
        this
    }

    pub fn as_bitboard(&self) -> u64 {
        let mut result = 0;
        for y in 0..8 {
            for x in 0..8 {
                result <<= 1;
                if self.atoms[x][y] {
                    result |= 1;
                }
            }
        }
        result
    }

    pub fn from_bitboard(bitboard: u64) -> Self {
        let mut this = Self::default();
        let mut bitboard = bitboard;
        for y in 0..8 {
            for x in 0..8 {
                this.atoms[7 - x][7 - y] = bitboard & 1 == 1;
                bitboard >>= 1;
            }
        }
        this
    }
}

impl Display for AtomGrid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        for y in 0..8 {
            for x in 0..8 {
                if self.atoms[x][y] {
                    f.write_str(" o")?;
                } else {
                    f.write_str(" .")?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Ensures, that the bitboard packing and unpacking works.
    #[test]
    fn test_bitboard() {
        for _ in 0..100 {
            let grid = AtomGrid::random(5);
            let bitboard = grid.as_bitboard();
            let grid2 = AtomGrid::from_bitboard(bitboard);
            assert_eq!(grid, grid2);
        }
    }
}
