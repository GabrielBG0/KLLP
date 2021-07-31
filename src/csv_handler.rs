use csv;
use native_windows_gui::stretch::node::Node;
use std::collections::HashMap;
use std::error::Error;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
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
    pub fn clean_print(self) {
        println!(
            "id: {}, word: {}, meaning: {}, views: {}, mastered: {}",
            self.id.unwrap(),
            self.word,
            self.meaning,
            self.views.unwrap(),
            self.mastered
        )
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

impl Default for Word {
    fn default() -> Self {
        Word {
            id: Some(0),
            word: String::new(),
            meaning: String::new(),
            views: Some(0),
            mastered: false,
        }
    }
}
