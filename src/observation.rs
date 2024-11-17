use crate::atom_grid::{AtomGrid, GRID_SIZE};
use crate::laser::Direction::*;
use crate::laser::{Direction, LaserTip};
use std::fmt::{Display, Formatter, Write};
use crate::i8vec2::I8Vec2;

/// The observation is the information derived from an atom grid using a laser and available to the
/// player. It is the player's job to use this information to determine the atom grid.
///
/// We store all the observations in a single struct and add to it after each probe.
#[derive(Debug)]
pub struct Observations {
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
    pub fn observe_all(grid: &AtomGrid) -> Self {
        let mut this = Observations::default();

        for direction in Direction::all() {
            for shift in 0..GRID_SIZE {
                if this.sides[direction as usize][shift] == NOT_PROBED {
                    this.probe(LaserTip::new(shift as u8, direction), grid);
                }
            }
        }

        this
    }

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

const ALPHABET: &'static str = "ABCDEFGHKLMNPRSTUVWYZ"; // Exclude some letters

impl Display for Observation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            NOT_PROBED => f.write_str("?"),
            LASER_ABSORBED => f.write_str("×"),
            LASER_REFLECTED => f.write_str("⇄"),
            Observation(3) => f.write_str("A"),
            Observation(i) => f.write_char(ALPHABET.chars().nth((i - 3) as usize).unwrap()),
        }
    }
}

pub fn draw(grid: &AtomGrid, observations: &Observations) -> Result<String, std::fmt::Error> {
    let mut f = String::new();
    // first, display the row above with lasers pointing down
    f.write_str("  ")?;
    for obs in observations.sides[Down as usize] {
        f.write_str(&format!(" {}", obs))?;
    }
    f.write_char('\n')?;

    let left_border = observations.sides[Right as usize];
    let right_border = observations.sides[Left as usize];

    // Show rows
    for y in 0..GRID_SIZE {
        let left_obs = left_border[y];
        let right_obs = right_border[y];

        f.write_str(&format!(" {}", left_obs))?;
        for x in 0..GRID_SIZE {
            if grid.get(I8Vec2::new(x as i8, y as i8)) {
                f.write_str(" o")?;
            } else {
                f.write_str(" .")?;
            }
        }
        f.write_str(&format!(" {}\n", right_obs))?;
    }

    f.write_str("  ")?;
    for obs in observations.sides[Up as usize] {
        f.write_str(&format!(" {}", obs))?;
    }
    f.write_char('\n')?;

    Ok(f)
}

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
