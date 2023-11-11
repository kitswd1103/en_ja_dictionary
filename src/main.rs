use std::env;

fn main() {
    let text = match env::args().skip(1).next() {
        Some(text) => text,
        _ => "".to_string()
    };

    for word in text_split(text) {
        println!("{}", word);
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
