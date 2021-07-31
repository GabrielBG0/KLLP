mod database_maneger;
use rusqlite::Connection;
use std::{fs::read_dir, panic};

fn main() {
    let conn = app_starter();
}

fn app_starter() -> Connection {
    let paths = read_dir("./").unwrap();
    let mut found = false;
    for path in paths {
        if path.unwrap().path().to_str().unwrap() == "./6kkw.db" {
            found = true;
            break;
        }
    }

    let conn = Connection::open("./6kkw.db").unwrap();
    if found == false {
        match database_maneger::create_database("6k Korean words.csv", &conn) {
            Ok(built) => {
                if built == true {
                    println!("Database built with success");
                    conn
                } else {
                    println!("Database has failed to build");
                    conn
                }
            }
            Err(e) => {
                panic!("Database failed to build with {:?}", e)
            }
        }
    } else {
        println!("Database already initialized");
        conn
    }
}
