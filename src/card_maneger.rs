#[path = "csv_handler.rs"]
mod csv_handler;
#[path = "./database_maneger.rs"]
mod database_maneger;
use csv_handler::Word;
use rand::prelude::*;
use rusqlite::Connection;
use std::error::Error;

pub fn get_words(quantity: u32, conn: &Connection) -> Result<Vec<Word>, Box<dyn Error>> {
    let mut words: Vec<database_maneger::Word> = Vec::new();

    for i in 0..quantity {
        let word = database_maneger::get_word(&i, conn)?;
        words.push(word);
    }

    Ok(words)
}
