//! Simple 2D integer vector based on i8.

use crate::atom_grid::GRID_SIZE;
use std::ops::{Add, Sub};

/// A simple 2D integer vector based on i8.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I8Vec2 {
    pub x: i8,
    pub y: i8,
}

impl I8Vec2 {
    /// Create a new vector.
    pub fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    pub fn in_grid(&self) -> bool {
        self.x >= 0 && self.x < 8 && self.y >= 0 && self.y < 8
    }

    pub fn random() -> Self {
        let x = rand::random::<usize>() % GRID_SIZE;
        let y = rand::random::<usize>() % GRID_SIZE;
        Self::new(x as i8, y as i8)
    }
}

impl Add for I8Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for I8Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
