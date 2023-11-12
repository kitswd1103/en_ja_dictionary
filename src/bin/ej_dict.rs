use std::env;

use en_ja_dictionary::{ej_dict::DictionaryDb, text_split};

fn main() {
    let text = match env::args().skip(1).next() {
        Some(text) => text,
        _ => "".to_string()
    };

    let open_db = DictionaryDb::open_db("");
    let db = open_db.unwrap();
    for word in text_split(text) {
        let items = (word.clone(), db.get_items(word.as_str()));
        println!("{}", items.0);
        for item in items.1 {
            println!("\t{}", item);
        }
    }
}
