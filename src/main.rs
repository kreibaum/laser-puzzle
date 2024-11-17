use crate::atom_grid::AtomGrid;
use crate::observation::Observations;

mod atom_grid;
mod i8vec2;
mod laser;
mod observation;

fn main() {
    let g = AtomGrid::random(5);

    println!("BitBoard: {}", g.as_bitboard());

    let o = Observations::observe_all(&g);
    println!("{}", observation::draw(&g, &o).expect("Failed to draw observation"));
}
