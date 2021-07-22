#[path = "csv_handler.rs"]
mod csv_handler;
use rusqlite::{Connection, Result};
use std::collections::HashMap;
use std::error::Error;

pub fn create_database(path: String, conn: &Connection) -> Result<bool, Box<dyn Error>> {
    conn.execute(
        "create table if not exists words (
            id integer primary key,
            word text not null,
            meaning text not null,
            views integer default 0,
            master boolean default false
        )",
        [],
    )?;

    let dict: HashMap<u32, csv_handler::Word> = csv_handler::csv_to_dict(&path).unwrap();

    for (key, value) in dict.values() {
        conn.execute(
            "inser in to words (id, word, meaning) values (?1, ?2, ?3)",
            &[&key, &value.word, &value.meaning],
        )?;
    }

    Ok(true)
}

pub fn get_word(id: &u32, conn: &Connection) -> Result<csv_handler::Word, Box<dyn Error>> {
    let get = conn.prepare("select * from word where id = ?")?;

    let word = get
        .query_map(&[id], |row| {
            Ok(csv_handler::Word {
                id: row.get(0)?,
                word: row.get(1)?,
                meaning: row.get(2)?,
                views: row.get(3)?,
                mastered: row.get(4)?,
            })
        })
        .unwrap();

    Ok(word.)
}
