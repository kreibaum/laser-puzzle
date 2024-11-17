use crate::atom_grid::{AtomGrid, GRID_SIZE};
use crate::i8vec2::I8Vec2;
use Direction::*;

/// We simulate the laser moving through the black box step by step. The laser starts at the border
/// of the box (so at "-1" or "max+1" coordinates). Its movement is only influenced by the three atoms
/// which may be positioned in front / front-left / front-right.
///
/// Those should be the only positions that can have any atoms at all. All other positions are
/// guaranteed to be empty. (Proof of this follows from the assumption that this held previously +
/// applying the movement rules)
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct LaserTip {
    position: I8Vec2,
    direction: Direction,
}

impl LaserTip {
    // Creating a new laser at the border of the box with a given shift and direction.
    pub fn new(shift: u8, direction: Direction) -> Self {
        match direction {
            Up => LaserTip {
                position: I8Vec2::new(shift as i8, GRID_SIZE as i8),
                direction,
            },
            Down => LaserTip {
                position: I8Vec2::new(shift as i8, -1),
                direction,
            },
            Left => LaserTip {
                position: I8Vec2::new(GRID_SIZE as i8, shift as i8),
                direction,
            },
            Right => LaserTip {
                position: I8Vec2::new(-1, shift as i8),
                direction,
            },
        }
    }

    /// Creates a new laser tip following the movement rules on the given atom grid.
    ///
    /// Rule 1: If there is an atom in front, be absorbed.
    ///  * o *
    ///  . ↑ .
    ///  . . .
    ///
    /// Rule 2: If there are no Atoms, move forward.
    ///  . ↑ .
    ///  . ↑ .
    ///  . . .
    ///
    /// Rule 3: If there are two atoms in front (both corners), be reflected.
    ///  o . o
    ///  . ↑ .
    ///  . ↓ .
    ///
    /// Rule 4+5: If there is an atom on the corner, turn and move to the side
    ///  o . .
    ///  . ↑ →
    ///  . . .
    pub fn move_once(self, grid: &AtomGrid) -> Option<Self> {
        // Rule 1. Afterward we can assume front == false.
        let front = self.position + self.direction.dxy();
        if grid.get(front) {
            return None;
        }
        let left = grid.get(front + self.direction.counter_clockwise().dxy());
        let right = grid.get(front + self.direction.clockwise().dxy());

        // Rule 2.
        if !left && !right {
            return Some(LaserTip {
                position: front,
                direction: self.direction,
            });
        }

        // Rule 3.
        if left && right {
            return Some(LaserTip {
                position: self.position + self.direction.flip().dxy(),
                direction: self.direction.flip(),
            });
        }

        // Rule 4.
        if left {
            return Some(LaserTip {
                position: self.position + self.direction.clockwise().dxy(),
                direction: self.direction.clockwise(),
            });
        }
        if right {
            return Some(LaserTip {
                position: self.position + self.direction.counter_clockwise().dxy(),
                direction: self.direction.counter_clockwise(),
            });
        }
        unreachable!("Logic error in laser movement. Movement rules not fully defined.")
    }

    pub fn traverse_grid(self, grid: &AtomGrid) -> Option<Self> {
        let mut laser = self;
        loop {
            laser = laser.move_once(grid)?; // ? handles absorption
            if !laser.position.in_grid() {
                return Some(laser);
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn dxy(self) -> I8Vec2 {
        match self {
            Up => I8Vec2::new(0, -1),
            Down => I8Vec2::new(0, 1),
            Left => I8Vec2::new(-1, 0),
            Right => I8Vec2::new(1, 0),
        }
    }

    fn clockwise(self) -> Self {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn counter_clockwise(self) -> Self {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    fn flip(self) -> Self {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_laser_path() {
        let grid = AtomGrid::from_bitboard(35184640598018);

        println!("{}", grid);

        // Watch it move a few steps
        let laser = LaserTip::new(0, Right);
        assert_eq!(laser.position, I8Vec2::new(-1, 0));
        let laser = laser.move_once(&grid).expect("moving possible");
        assert_eq!(laser.position, I8Vec2::new(0, 0));
        let laser = laser.move_once(&grid).expect("moving possible");
        assert_eq!(laser.position, I8Vec2::new(1, 0));

        // Restart the laser and let it traverse the grid
        let laser = LaserTip::new(0, Right);
        let laser = laser.traverse_grid(&grid).expect("traversal possible");
        assert_eq!(laser.position, I8Vec2::new(8, 0));

        // When shining the laser in the second row, we expect a reflection towards the top
        let laser = LaserTip::new(1, Right);
        let laser = laser.traverse_grid(&grid).expect("traversal possible");
        assert_eq!(laser.position, I8Vec2::new(1, -1));

        // Next laser is absorbed
        let laser = LaserTip::new(2, Right);
        let laser = laser.traverse_grid(&grid);
        assert_eq!(laser, None);

        // Next laser leaves the grid on the left side, but further down. It was reflected twice.
        let laser = LaserTip::new(3, Right);
        let laser = laser.traverse_grid(&grid).expect("traversal possible");
        assert_eq!(laser.position, I8Vec2::new(-1, 5));

        // On y=4 the laser is absorbed again
        let laser = LaserTip::new(4, Right);
        let laser = laser.traverse_grid(&grid);
        assert_eq!(laser, None);

        // For y=5 the laser comes back to y=3 by symmetry
        let laser = LaserTip::new(5, Right);
        let laser = laser.traverse_grid(&grid).expect("traversal possible");
        assert_eq!(laser.position, I8Vec2::new(-1, 3));

        // For y=6 the laser is absorbed again
        let laser = LaserTip::new(6, Right);
        let laser = laser.traverse_grid(&grid);
        assert_eq!(laser, None);

        // For y=7 the laser is reflected down.
        let laser = LaserTip::new(7, Right);
        let laser = laser.traverse_grid(&grid).expect("traversal possible");
        assert_eq!(laser.position, I8Vec2::new(1, 8));
    }
}