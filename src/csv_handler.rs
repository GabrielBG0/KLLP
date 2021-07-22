use csv;
use std::collections::HashMap;
use std::error::Error;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Word {
    pub id: Option<u32>,
    pub word: String,
    pub meaning: String,
    pub views: Option<u32>,
    pub mastered: bool,
}

impl Word {
    pub fn create_from_new(word: String, meaning: String) -> Self {
        Word {
            id: None,
            word,
            meaning,
            views: None,
            mastered: false,
        }
    }

    pub fn create_from_db(
        id: u32,
        word: String,
        meaning: String,
        views: u32,
        mastered: bool,
    ) -> Self {
        Word {
            id: Some(id),
            word,
            meaning,
            views: Some(views),
            mastered,
        }
    }
}

pub fn csv_to_dict(path: &str) -> Result<HashMap<u32, Word>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;
    let mut dict: HashMap<u32, Word> = HashMap::new();

    for result in rdr.records() {
        let record = result?;
        dict.insert(
            record.get(0).unwrap().parse::<u32>().unwrap(),
            Word::create_from_new(
                record.get(1).unwrap().to_string(),
                record.get(2).unwrap().to_string(),
            ),
        );
    }
    Ok(dict)
}
