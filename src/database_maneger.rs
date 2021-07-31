#[path = "csv_handler.rs"]
mod csv_handler;
use rusqlite::{params, Connection};
use std::collections::HashMap;
use std::error::Error;

use self::csv_handler::Word;

pub fn create_database(path: &str, conn: &Connection) -> Result<bool, Box<dyn Error>> {
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

    let dict: HashMap<u32, csv_handler::Word> = csv_handler::csv_to_dict(path).unwrap();

    for (key, value) in dict {
        conn.execute(
            "insert into words (id, word, meaning) values (?1, ?2, ?3)",
            params![&key, &value.word, &value.meaning],
        )?;
    }

    Ok(true)
}

fn build_word(
    id: u32,
    word: String,
    meaning: String,
    views: u32,
    mastered: bool,
) -> Result<Word, Box<dyn Error>> {
    Ok(csv_handler::Word::create_from_db(
        id, word, meaning, views, mastered,
    ))
}

pub fn get_word(id: &u32, conn: &Connection) -> Result<Option<csv_handler::Word>, Box<dyn Error>> {
    let mut stmt = conn.prepare("select * from words where id = ?")?;

    let rows = stmt.query_and_then(params![id], |row| {
        build_word(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
        )
    })?;

    let mut words = Vec::new();
    for word in rows {
        words.push(word.unwrap())
    }

    match words.get(0) {
        Some(word) => return Ok(Some(word.clone())),
        None => return Ok(None),
    }
}
