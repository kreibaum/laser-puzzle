use std::fmt::{Display, Formatter};

/// The hidden inner secret of the game
#[derive(Default, Debug)]
struct AtomGrid {
    atoms: [[bool; 8]; 8],
}

impl AtomGrid {
    pub fn get(&self, x: u8, y: u8) -> bool {
        self.atoms[x as usize][y as usize]
    }
    pub fn set(&mut self, x: u8, y: u8, value: bool) {
        self.atoms[x as usize][y as usize] = value;
    }

    pub fn random(atom_count: u8) -> Self {
        let mut this = Self::default();
        let mut placed_down = 0;
        while placed_down < atom_count {
            let x = rand::random::<u8>() % 8;
            let y = rand::random::<u8>() % 8;
            if !this.get(x, y) {
                this.set(x, y, true);
                placed_down += 1;
            }
        }
        this
    }
}

impl Display for AtomGrid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        for y in 0..8 {
            for x in 0..8 {
                if self.get(x, y) {
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

fn main() {
    let g = AtomGrid::random(5);

    println!("{}", g);
}
