use crate::atom_grid::{AtomGrid, GRID_SIZE};
use crate::laser::LaserTip;

/// The observation is the information derived from an atom grid using a laser and available to the
/// player. It is the player's job to use this information to determine the atom grid.
///
/// We store all the observations in a single struct and add to it after each probe.
struct Observations {
    next_observation: Observation,
    sides: [[Observation; GRID_SIZE]; 4],
}

impl Default for Observations {
    fn default() -> Self {
        Observations {
            next_observation: Observation(3), // We start at 3 as 0-2 have special significance.
            sides: [[NOT_PROBED; GRID_SIZE]; 4],
        }
    }
}

impl Observations {
    fn probe(&mut self, laser: LaserTip, grid: &AtomGrid) {
        let (in_shift, in_direction) = laser
            .deconstruct()
            .expect("Probing should only happen with side-lasers.");
        let (laser_out, move_count) = laser.traverse_grid(grid);

        if move_count <= 1 && laser_out.is_some() {
            // Reflection
            self.sides[in_direction as usize][in_shift as usize] = LASER_REFLECTED;
        } else if let Some(laser_out) = laser_out {
            // Laser came out somewhere else
            let (out_shift, out_direction) = laser_out
                .deconstruct()
                .expect("Traversal should return the laser on the border.");
            if in_direction == out_direction && in_shift == out_shift {
                self.sides[in_direction as usize][in_shift as usize] = LASER_REFLECTED;
            } else {
                self.sides[in_direction as usize][in_shift as usize] = self.next_observation;
                self.sides[out_direction as usize][out_shift as usize] = self.next_observation;
                self.next_observation = Observation(self.next_observation.0 + 1);
            }
        } else {
            // Laser absorbed
            self.sides[in_direction as usize][in_shift as usize] = LASER_ABSORBED;
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Observation(u8);

const NOT_PROBED: Observation = Observation(0); // Special value
const LASER_ABSORBED: Observation = Observation(1); // Special value
const LASER_REFLECTED: Observation = Observation(2); // Special value

#[cfg(test)]
mod tests {
    use crate::atom_grid::AtomGrid;
    use crate::laser::Direction::*;
    use crate::laser::LaserTip;
    use crate::observation::{Observations, LASER_ABSORBED, LASER_REFLECTED};

    #[test]
    fn observation_after_probing() {
        let grid = AtomGrid::from_bitboard(54043333103714304);
        println!("{}", grid);

        let mut observations = Observations::default();

        // Probe along the left side, shooting lasers to the right.
        for i in 0..8 {
            observations.probe(LaserTip::new(i, Right), &grid);
        }

        let obs = observations.sides[Right as usize];
        println!("{:?}", obs);
        assert_eq!(obs[0], LASER_REFLECTED);
        assert_eq!(obs[1], LASER_ABSORBED);
        assert_eq!(obs[2], LASER_REFLECTED);
        assert_eq!(obs[3], LASER_ABSORBED);
        assert_eq!(obs[4], LASER_REFLECTED);
        assert_eq!(obs[5], LASER_ABSORBED);
        assert_eq!(obs[6].0, 3);
        assert_eq!(obs[7].0, 4);
    }
}
