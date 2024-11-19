use itertools::Itertools;
use rusqlite::{params, Connection, Result};
use crate::atom_grid::AtomGrid;
use crate::i8vec2::I8Vec2;
use crate::observation::Observations;
use std::fmt::Write;


mod atom_grid;
mod i8vec2;
mod laser;
mod observation;

fn setup_database() -> Result<()> {
    let conn = Connection::open("solutions.db")?;
    conn.execute("DROP TABLE IF EXISTS HiddenStateObservation", [])?;
    conn.execute(
       "CREATE TABLE IF NOT EXISTS HiddenStateObservation (
            ID INTEGER PRIMARY KEY,
            HiddenState TEXT NOT NULL,
            Observation TEXT NOT NULL
        )", [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_HiddenState ON HiddenStateObservation (HiddenState)", [] )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_Observation ON HiddenStateObservation (Observation)", [] )?;

    Ok(())
}

fn insert_state(conn: &Connection, hidden_state: &str, observation: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO HiddenStateObservation (HiddenState, Observation) VALUES (?1, ?2)",
        params![hidden_state, observation],
    )?;

    Ok(())
}

fn get_all_hidden_states(conn: &Connection, observation: &str) -> Result<Vec<String>>{
    let mut query = conn.prepare("SELECT HiddenState FROM HiddenStateObservation WHERE Observation = ?")?;
    let hidden_state_iter = query.query_map(params![observation], |row| {row.get(0)})?;
    let mut hidden_states = Vec::new();
    for state in hidden_state_iter {
        hidden_states.push(state?);
    }   
    Ok(hidden_states)
}

fn generate_all_grids(conn: &Connection) -> Result<()> {
    for combination in (0..7).combinations(5){ // TODO: change 7 back to 64
        let mut grid = AtomGrid::default(); // Creates an empty board
        for ele in &combination {
            grid.set(I8Vec2::new(ele % 8, ele / 8), true);
        }
        println!("{:?}", combination);

        let grid_bb = format!("{}", grid.as_bitboard());
        let obs = Observations::observe_all(&grid);
        let mut obs_string : String = "".to_string();
        for (_, _, obs) in obs.iter() {
            obs_string.write_str(&format!("{}", obs));
        }
        // println!("{}, {}", grid_bb, obs_string);

        // Insert into database
        insert_state(conn, &grid_bb, &obs_string)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    match setup_database(){
        Ok(_) => println!("Database setup"),
        Err(e) => println!("Error database setup: {}", e)
    }

    let conn = Connection::open("solutions.db")?;

    generate_all_grids(&conn)?;
    println!("Data insertion done");

    let hidden_states = get_all_hidden_states(&conn, "AB×××××CA⇄×××××⇄×CDEFGHK×BDEFGHK")?;
    println!("Hidden States:");
    for state in hidden_states {
        println!("{}", state);
    }

    Ok(())
}