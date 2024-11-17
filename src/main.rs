use crate::atom_grid::AtomGrid;

mod atom_grid;
mod i8vec2;
mod laser;
mod observation;

fn main() {
    let g = AtomGrid::random(5);

    println!("{}", g);
    println!("BitBoard: {}", g.as_bitboard());
}
