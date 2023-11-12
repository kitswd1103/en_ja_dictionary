
use std::fmt::Display;

use rusqlite::{Connection, params};

pub struct DictionaryDb {
    db: Connection,
}

impl DictionaryDb  {
    const QUERY_SEARCH_FORMAT: &str = r"SELECT word, MAX(mean), level, MAX(user_mean)
        FROM (
            SELECT word, mean, level, NULL as user_mean FROM items WHERE word LIKE '%__SEARCH_WORD__%'
            UNION ALL
            SELECT word, NULL as mean, level, mean as user_mean FROM user WHERE word LIKE '%__SEARCH_WORD__%'
        ) GROUP BY word
        ORDER BY CASE WHEN word = '__SEARCH_WORD__' THEN 0 ELSE 1 END, level DESC
        LIMIT 3";
    const QUERY_SEACH_REPLACE_WORD: &str = "__SEARCH_WORD__";

    fn create_query_search(word: &str) -> String {
        Self::QUERY_SEARCH_FORMAT.replace(Self::QUERY_SEACH_REPLACE_WORD, word)
    }
    
    pub fn open_db(path: &str) -> rusqlite::Result<Self> {
        let path = 
            if path.is_empty() { "./db/ejdict.sqlite3" }
            else { path };

        let ret = Self { db: Connection::open(path)? };
        ret.create_user_table()?;
        Ok(ret)
    }

    pub fn create_user_table(&self) -> rusqlite::Result<()> {
        self.db.execute(
            r"CREATE TABLE IF NOT EXISTS user (
                id INTEGER PRIMARY KEY,
                word TEXT UNIQUE,
                mean TEXT NOT NULL,
                level INTEGER NOT NULL)", [])?;
        Ok(())
    }

    pub fn upsert_word(&self, item: DictionaryItem) -> rusqlite::Result<()> {
        self.db.execute(
            r"INSERT INTO user (word, mean, level) VALUES (?, ?, ?)
            ON CONFLICT(word) DO UPDATE SET word=excluded.word, mean=excluded.mean, level=excluded.level",
            params![item.word, item.user_mean, item.level])?;
        Ok(())
    }

    pub fn delete_word(&self, word: &str) -> rusqlite::Result<()>{
        self.db.execute(
            "DELETE FROM user WHERE word = ? COLLATE NOCASE",
            [word],
        )?;
        Ok(())
    }

    pub fn get_items(&self, word: &str) -> rusqlite::Result<Vec<DictionaryItem>> {
        let mut items = self.db.prepare(Self::create_query_search(word).as_str() ).unwrap();
        let items = items.query_map([], |row| { 
            Ok(DictionaryItem {
                word: row.get(0).unwrap_or_default(),
                mean: row.get(1).unwrap_or_default(),
                level: row.get(2).unwrap_or_default(),
                user_mean: row.get(3).unwrap_or_default()
            })
        })?;

        Ok(items.map(|item| item.unwrap()).collect())
    }

    pub fn display_user_table_contents(&self) -> rusqlite::Result<()> {
        let mut stmt = self.db.prepare("SELECT * FROM user")?;
        let rows = stmt.query_map([], |row| {
            Ok(DictionaryItem {
                word: row.get(1).unwrap_or_default(),
                mean: row.get(2).unwrap_or_default(),
                level: row.get(3).unwrap_or_default(),
                ..Default::default()
            })
        })?;
        
        for row in rows {
            let row = row?;
            println!("word: {}, mean: {}, level: {}", row.word, row.mean, row.level);
        }
    
        Ok(())
    }
}

#[derive(Default)]
pub struct DictionaryItem {
    pub word: String,
    pub mean: String,
    pub level: u32,
    pub user_mean: String,
}

impl Display for DictionaryItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "word: {}, mean: {}, level: {}, user_mean: {}", self.word, self.mean, self.level, self.user_mean)
    }
}

#[cfg(test)]
mod test {
    use super::{DictionaryDb, DictionaryItem};

    #[test]
    fn test_upsert_delete() {
        let db = DictionaryDb::open_db("").expect("データベースの読み込み失敗");
        let item = DictionaryItem {
            word: "Hoge".into(),
            user_mean: "hoge hoge".into(),
            ..Default::default()
        };
        db.upsert_word(item).expect("挿入失敗");
        let items = db.get_items("Hoge").expect("データ取得失敗");

        assert!(items.iter().any(|item| item.word == "Hoge" && item.user_mean == "hoge hoge" ));

        for item in items{
            println!("{}", item);
        }

        db.delete_word("hoge").expect("データ削除失敗");

        let items = db.get_items("Hoge").expect("データ取得失敗");
        assert!(items.iter().all(|item| item.word != "Hoge" && item.user_mean != "hoge hoge" ));
        for item in items{
            println!("{}", item);
        }
    }
}
