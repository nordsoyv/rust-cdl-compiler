use std::iter::Peekable;

pub fn lex_selector(selector: &str) -> Vec<LexItem> {
    let mut it = selector.chars().peekable();
    let mut result = Vec::new();
    while let Some(&c) = it.peek() {
        match c {
            'a'...'z' | 'A'...'Z' | '_' => {
                it.next();
                let ident = get_identifier(c, &mut it);
                result.push(LexItem::Identifier(ident));
            }
            '.' => {
                result.push(LexItem::Dot);
                it.next();
            }
            '[' => {
                result.push(LexItem::OpenSquare);
                it.next();
            }
            ']' => {
                result.push(LexItem::CloseSquare);
                it.next();
            }
            '>' => {
                result.push(LexItem::Arrow);
                it.next();
            }
            ' ' => {
                it.next();
            }
            _ => {
                println!("Unknown parsing {}", c);
                it.next();
            }
        }
    }
    return result;
}


#[derive(Debug, PartialEq)]
pub enum LexItem {
    Identifier(String),
    Dot,
    OpenSquare,
    CloseSquare,
    Arrow,
}


fn get_identifier<T: Iterator<Item=char>>(c: char, iter: &mut Peekable<T>) -> String {
    let mut identifier = String::new();
    identifier.push(c);
    while let Some(&ch) = iter.peek() {
        match ch {
            'a'...'z' | 'A'...'Z' | '_' | '0'...'9' => {
                identifier.push(ch);
                iter.next();
            }
            _ => { break; }
        }
    }
    identifier
}



#[cfg(test)]
mod test {
    use select::lex::lex_selector;

    #[test]
    fn lex_selector_test() {
        let s = "main[subType].identifier";
        let selector = lex_selector(s);
        assert_eq!(selector.len(), 6);
    }

    #[test]
    fn lex_selector_test2() {
        let s = "main[subType].identifier > main";
        let selector = lex_selector(s);
        assert_eq!(selector.len(), 8);
    }

}