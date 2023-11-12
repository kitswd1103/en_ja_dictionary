
use std::fmt::Display;

use rusqlite::Connection;

pub struct DictionaryDb {
    db: Connection,
}

impl DictionaryDb  {
    const QUERY_FORMAT: &str = r"SELECT word, mean, level FROM items
    WHERE word LIKE '%__SEARCH_WORD__%'
    ORDER BY CASE WHEN word = '__SEARCH_WORD__' THEN 0 ELSE 1 END, level DESC
    LIMIT 3";
    const QUERY_REPLACE_WORD: &str = "__SEARCH_WORD__";

    pub fn open_db(path: &str) -> Result<Self, rusqlite::Error> {
        let path = 
            if path.is_empty() { "./db/ejdict.sqlite3" }
            else { path };

        Ok(Self { db: Connection::open(&path)? })
    }

    pub fn get_items(&self, word: &str) -> Vec<DictionaryItem> {
        let mut items = self.db.prepare(Self::QUERY_FORMAT.replace(Self::QUERY_REPLACE_WORD, word).as_str()).unwrap();
        let items = items.query_map([], |row| { 
            let word = row.get(0);
            let mean = row.get(1);
            let level = row.get(2);
            Ok(DictionaryItem {
                word: word.unwrap_or(String::default()),
                mean: mean.unwrap_or(String::default()),
                level: level.unwrap_or(0),
            })
        }).unwrap();

        items.map(|item| item.unwrap()).collect()
    }
}

pub struct DictionaryItem {
    pub word: String,
    pub mean: String,
    pub level: u32,
}

impl Display for DictionaryItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "word: {}, mean: {}, level: {}", self.word, self.mean, self.level)
    }
}
