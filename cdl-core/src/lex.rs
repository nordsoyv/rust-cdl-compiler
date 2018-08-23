use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum LexItem {
    Identifier(String),
    String(String),
    Colon,
    OpenBracket,
    CloseBracket,
    EOL,
}

#[derive(Debug)]
pub struct TokenStream {
    tokens : Vec<LexItem>,
    index : usize,
}

#[allow(dead_code)]
impl TokenStream {
    pub fn get_length(&self)-> usize {
        self.tokens.len()
    }

    pub fn get_current_token (&self) -> &LexItem{
        &self.tokens[self.index]
    }

    pub fn get_next_token  (&self) -> &LexItem {
        if self.index + 1 < self.tokens.len() {
            return &self.tokens[self.index+1]
        }
        panic!("Trying to access token past end of stream")
    }

    pub fn advance_stream(&mut self){
        self.index += 1;
    }

    pub fn has_items(&self) -> bool{
        self.index < self.tokens.len()
    }

}



pub fn lex(input: &String) -> Result<TokenStream, String> {
    let mut result = Vec::new();
    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            'a'...'z' => {
                it.next();
                let ident = get_identifier(c, &mut it);
                result.push(LexItem::Identifier(ident));
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
//                println!("End of line! n");
                result.push(LexItem::EOL);
                it.next();
            }
            ' ' => {
//                println!("Found space '{}'", c);
                it.next();
            }
            '"' => {
                it.next();
                let quoted = get_quoted_string( &mut it);
                result.push(LexItem::String(quoted));
            }
            _ => { println!("Unknown parsing {}", c); it.next();}
        }
    }

    let stream = TokenStream{
        tokens: result,
        index: 0,
    };
    Ok(stream)
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
            _ => {break; }
        }
    }
    identifier
}

fn get_quoted_string<T: Iterator<Item=char>>(iter: &mut Peekable<T>) -> String {
    let mut quoted = String::new();
    while let Some(&ch) = iter.peek() {
        match ch {
            '"' => {iter.next(); break; }
            _ => {
                quoted.push(ch);
                iter.next();
            }
        }
    }
    quoted
}
