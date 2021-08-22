#[path = "csv_handler.rs"]
use csv_handler::*;
use indicatif::{ProgressBar, ProgressStyle};
use rusqlite::{named_params, params, Connection};
use std::{collections::HashMap, error::Error, fs::remove_file};

pub fn create_database(path: &str, conn: &Connection) -> Result<bool, Box<dyn Error>> {
    match conn.execute(
        "create table if not exists words (
            id integer primary key,
            word text not null,
            meaning text not null,
            views integer default 0,
            master boolean default false
        )",
        [],
    ) {
        Ok(o) => println!("Database createde with success"),
        Err(e) => {
            println!("Couldn't create database, Error: {:?}", e);
            return Ok(false);
        }
    }

    let dict: HashMap<u32, csv_handler::Word> = csv_handler::csv_to_dict(path)
        .unwrap_or_else(|error| panic!("file read faild with {:?}", error));

    let pb = ProgressBar::new(dict.len() as u64);

    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.red} [{elapsed_precise}] [{wide_bar:.blue}] {percent}% ({eta})")
            .progress_chars("█▋ "),
    );

    println!("Addinng words to database:");
    for (key, value) in dict {
        match conn.execute(
            "insert into words (id, word, meaning) values (?1, ?2, ?3)",
            params![&key, &value.word, &value.meaning],
        ) {
            Ok(o) => pb.inc(1),
            Err(e) => {
                println!("Sistem faild to inser a word.");
                println!("Deleting database.");
                remove_file("./6kkw.db").unwrap_or_else(|e2| {
                    panic!(
                        "Database deletion faild.\n Error stacktrace was:\n {:?}\n{:?}",
                        e, e2
                    )
                });
            }
        }
    }
    pb.finish_with_message("All words added to database");

    Ok(true)
}

fn build_word(
    id: u32,
    word: String,
    meaning: String,
    views: u32,
    mastered: bool,
) -> Result<csv_handler::Word, Box<dyn Error>> {
    Ok(csv_handler::Word::create_from_db(
        id, word, meaning, views, mastered,
    ))
}

pub fn get_word(id: &u32, conn: &Connection) -> Result<csv_handler::Word, Box<dyn Error>> {
    let mut stmt = conn.prepare("select * from words where id = ?")?;

    let word = stmt.query_row(params![id], |row| {
        Ok(build_word(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
        ))
    })??;

    Ok(word)
}

pub fn add_view(id: &u32, conn: &Connection) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare("select views from words where id = ?")?;

    let views: u32 = stmt.query_row(params![id], |row| Ok(row.get(0)?))?;

    stmt = conn.prepare("update words set views = :v where id = :id ")?;

    stmt.execute(named_params! {":v": views + 1, ":id": id})?;

    Ok(())
}
