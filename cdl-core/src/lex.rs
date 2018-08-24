use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum LexItem {
    Identifier(String),
    String(String),
    Reference(String),
    Colon,
    OpenBracket,
    CloseBracket,
    OpenPar,
    ClosePar,
    Plus,
    Minus,
    Div,
    Mul,
    EOL,
}

pub struct Lexer {
    input: String
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
        }
    }

    pub fn lex(&self) -> Result<Vec<LexItem>, String> {
        let mut result = Vec::new();
        let mut it = self.input.chars().peekable();
        while let Some(&c) = it.peek() {
            match c {
                'a'...'z' => {
                    it.next();
                    let ident = get_identifier(c, &mut it);
                    result.push(LexItem::Identifier(ident));
                }
                '@' => {
                    it.next();
                    let reference = get_reference(&mut it);
                    result.push(LexItem::Reference(reference));
                }
                '{' => {
                    result.push(LexItem::OpenBracket);
                    it.next();
                }
                '}' => {
                    result.push(LexItem::CloseBracket);
                    it.next();
                }
                ':' => {
                    result.push(LexItem::Colon);
                    it.next();
                }
                '\n' => {
                    result.push(LexItem::EOL);
                    it.next();
                }
                ' ' => {
                    it.next();
                }
                '"' => {
                    it.next();
                    let quoted = get_quoted_string(&mut it);
                    result.push(LexItem::String(quoted));
                }
                '(' => {
                    result.push(LexItem::OpenPar);
                    it.next();
                }
                ')' => {
                    result.push(LexItem::ClosePar);
                    it.next();
                }
                '+' => {
                    result.push(LexItem::Plus);
                    it.next();
                }
                '-' => {
                    result.push(LexItem::Minus);
                    it.next();
                }
                '/' => {
                    result.push(LexItem::Div);
                    it.next();
                }
                '*' => {
                    result.push(LexItem::Mul);
                    it.next();
                }
                _ => {
                    println!("Unknown parsing {}", c);
                    it.next();
                }
            }
        }
        Ok(result)
    }
}

fn get_identifier<T: Iterator<Item=char>>(c: char, iter: &mut Peekable<T>) -> String {
    let mut identifier = String::new();
    identifier.push(c);
    while let Some(&ch) = iter.peek() {
        match ch {
            'a'...'z' => {
                identifier.push(ch);
                iter.next();
            }
            _ => { break; }
        }
    }
    identifier
}

fn get_reference<T: Iterator<Item=char>>(iter: &mut Peekable<T>) -> String {
    let mut reference = String::new();
    while let Some(&ch) = iter.peek() {
        match ch {
            'a'...'z' | '.' => {
                reference.push(ch);
                iter.next();
            }
            _ => { break; }
        }
    }
    reference
}


fn get_quoted_string<T: Iterator<Item=char>>(iter: &mut Peekable<T>) -> String {
    let mut quoted = String::new();
    while let Some(&ch) = iter.peek() {
        match ch {
            '"' => {
                iter.next();
                break;
            }
            _ => {
                quoted.push(ch);
                iter.next();
            }
        }
    }
    quoted
}
