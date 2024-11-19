use rusqlite::{params, Connection, Result};


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

fn main(){
    match setup_database(){
        Ok(_) => println!("Database setup"),
        Err(e) => println!("Error database setup: {}", e)
    }

    let conn = match Connection::open("solutions.db") {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error opening database: {}", e);
            return;
        }
    };
    
    insert_state(&conn, "01-02-03-04-05", "32stringgoeshere");
    insert_state(&conn, "02-02-03-04-05", "32stringgoeshere");

    match get_all_hidden_states(&conn, "32stringgoeshere") {
        Ok(hidden_states) => {
            println!("Hidden States:");
            for state in hidden_states {
                println!("{}", state);
            }
        },
        Err(e) => println!("Error fetching hidden states: {}", e),
    }
    println!("Sample data insertion done");
}