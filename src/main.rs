use crate::atom_grid::AtomGrid;
use crate::observation::Observations;

mod atom_grid;
mod i8vec2;
mod laser;
mod observation;
mod solver;

fn main() {
    let g = AtomGrid::random(5);

    println!("BitBoard: {}", g.as_bitboard());

    let o = Observations::observe_all(&g);
    println!(
        "{}",
        observation::draw(&g, &o).expect("Failed to draw observation")
    );

    let s = solver::solve_as_much_as_you_can(&o);
    println!(
        "{}",
        solver::draw(&s, &o).expect("Failed to draw solver state")
    );
}
