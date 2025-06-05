// sqlite_schema_discovery.rs
use rusqlite::{Connection, Result};
use std::env;

fn main() -> Result<()> {
    // Get the database path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <database-path>");
        std::process::exit(1);
    }

    let db_path = &args[1];

    // Open the SQLite database
    let conn = Connection::open(db_path)?;

    // Query all tables from the sqlite_master system table
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type = 'table'")?;
    let table_names = stmt.query_map([], |row| row.get::<_, String>(0))?;

    println!("Schema for database: {}", db_path);
    println!("----------------------------");

    for table_name in table_names {
        let table_name = table_name?;
        let sql = get_table_schema(&conn, &table_name)?;
        println!("-- Schema for table '{}':", table_name);
        println!("{}", sql);
        println!();
    }

    Ok(())
}

/// Retrieve the CREATE statement for a specific table
fn get_table_schema(conn: &Connection, table_name: &str) -> Result<String> {
    let mut stmt =
        conn.prepare("SELECT sql FROM sqlite_master WHERE type = 'table' AND name = ?1")?;
    let mut rows = stmt.query([table_name])?;

    if let Some(row) = rows.next()? {
        let sql: String = row.get(0)?;
        Ok(sql)
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}
