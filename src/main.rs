use std::{env, fmt::Display};

use rusqlite::Connection;

struct DictionaryDb {
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

struct DictionaryItem {
    pub word: String,
    pub mean: String,
    pub level: u32,
}

impl Display for DictionaryItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "word: {}, mean: {}, level: {}", self.word, self.mean, self.level)
    }
}

fn main() {
    let text = match env::args().skip(1).next() {
        Some(text) => text,
        _ => "".to_string()
    };

    let db = DictionaryDb::open_db("").unwrap();
    for word in text_split(text) {
        let items = db.get_items(word.as_str());
        for item in items {
            println!("\t{}", item);
        }
    }
}

fn text_split(text: String) -> Vec<String> {
    let mut ret = Vec::new();

    let mut temp = String::new();
    let mut in_quote = false;

    for c in text.chars() {
        if c == '\"' {
            if !temp.is_empty() {
                ret.push(temp);
                temp = "".to_string();
            }
            in_quote = !in_quote;
            continue;
        }
        if is_delimiter(c) && !in_quote {
            if !temp.is_empty() {
                ret.push(temp);
                temp = "".to_string();
            }
            continue;
        }
        temp.push(c);
    }
    if !temp.is_empty() {
        ret.push(temp);
    }
    ret
}
fn is_delimiter(c: char) -> bool {
    c == ' ' || c == ',' || c == '.' || c == '\"'
}

#[cfg(test)]
mod test {
    use crate::text_split;

    #[test]
    fn test_text_split() {
        let text = "aaa bbb, ccc. ddd.  , eee".to_string();
        let words = text_split(text);
        assert_eq!(words, ["aaa", "bbb", "ccc", "ddd", "eee"].into_iter().map(str::to_string).collect::<Vec<_>>());
        let text = "aaa \"bbb, ccc.\" ddd\".  , \"eee".to_string();
        let words = text_split(text);
        assert_eq!(words, ["aaa", "bbb, ccc.", "ddd", ".  , ", "eee"].into_iter().map(str::to_string).collect::<Vec<_>>());
 
    }
}
