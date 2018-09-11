use std::iter::Peekable;
use std::cell::RefCell;
use std::cell::Cell;
use std::cell::Ref;
use parse::AstEntityNode;
use parse::AstRootNode;
use parse::AstEntityHeaderNode;
use parse::AstFieldNode;

pub fn select_entity<'a>(root: &'a AstRootNode, selector_string: &str) -> Vec<&'a AstEntityNode> {
    let tokens = lex_selector(selector_string);
    let parser = SelectorParser {
        tokens: RefCell::from(tokens),
        index: Cell::from(0),
    };

    let selector = parser.parse().unwrap();

    let mut result = vec![];

    for child in &root.children {
        let mut sub_results = select_in_entity(child, &selector);
        if sub_results.len() > 0 {
            result.append(&mut sub_results);
        }
    }

    return result;
}

pub fn select_field<'a>(root: &'a AstRootNode, selector_string : &str) -> Vec<&'a AstFieldNode> {
    let tokens = lex_selector(selector_string);
    let parser = SelectorParser {
        tokens: RefCell::from(tokens),
        index: Cell::from(0),
    };

    let selector = parser.parse().unwrap();

    let mut result = vec![];
    for child in &root.children {
        let mut sub_results = select_field_in_entity(child, &selector);
        if sub_results.len() > 0 {
            result.append(&mut sub_results);
        }
    }

    return result;

}

fn select_in_entity<'a>(entity: &'a AstEntityNode, selector: &Selector) -> Vec<&'a AstEntityNode> {
    let mut result = vec![];

    if matches_selector(&entity.header, &selector) {
        result.push(entity);
    }

    for child in &entity.body.children {
        let mut sub_results = select_in_entity(child, selector);
        if sub_results.len() > 0 {
            result.append(&mut sub_results);
        }
    }
    return result;
}

fn select_field_in_entity<'a>(entity: &'a AstEntityNode, selector: &Selector) -> Vec<&'a AstFieldNode> {
    let mut result = vec![];
    for field in &entity.body.fields {
        match selector.identifier {
            Some(ref id) => {
                if id == &field.identifier {
                    result.push(field);
                }
            }
            None => {}
        }
    }

    for child in &entity.body.children {
        let mut sub_results = select_field_in_entity(child, &selector);
        if sub_results.len() > 0 {
            result.append(&mut sub_results);
        }
    }

    return result;
}

fn matches_selector(header: &AstEntityHeaderNode, selector: &Selector) -> bool {
    let mut matches = true;
    match selector.main_type {
        Some(ref s) => {
            if &header.main_type != s {
                matches = false;
            }
        }
        None => {}
    }
    match selector.sub_type {
        Some(ref s) => {
            match header.sub_type {
                Some(ref hs) => {
                    if hs != s {
                        matches = false;
                    }
                }
                None => {
                    // matching on an sub_type, but entity has none, no match
                    matches = false;
                }
            }
        }
        None => {}
    }
    match selector.identifier {
        Some(ref s) => {
            match header.identifier {
                Some(ref hi) => {
                    if hi != s {
                        matches = false;
                    }
                }
                None => {
                    // matching on an identifier, but entity has none, no match
                    matches = false;
                }
            }
        }
        None => {}
    }

    return matches;
}

#[derive(Debug)]
struct Selector {
    pub main_type: Option<String>,
    pub sub_type: Option<String>,
    pub identifier: Option<String>,
    pub child: Option<Box<Selector>>,
}

#[derive(Debug)]
struct SelectorParser {
    tokens: RefCell<Vec<LexItem>>,
    index: Cell<usize>,
}

impl SelectorParser {
    fn get_length(&self) -> usize {
        self.tokens.borrow().len()
    }

    fn peek_current_token(&self) -> Ref<LexItem> {
        Ref::map(self.tokens.borrow(), |tokens| &tokens[self.index.get()])
    }

    fn peek_next_token(&self) -> Result<Ref<LexItem>, String> {
        if self.index.get() + 1 <= self.tokens.borrow().len() {
            return Ok(Ref::map(self.tokens.borrow(), |tokens| &tokens[self.index.get() + 1]));
        }
        Err(format!("Trying to access token past end of stream"))
    }

    fn get_current_token(&self) -> Ref<LexItem> {
        self.advance_stream();
        Ref::map(self.tokens.borrow(), |tokens| &tokens[self.index.get() - 1])
    }

    fn advance_stream(&self) {
        if self.index.get() + 1 <= self.tokens.borrow().len() {
            self.index.set(self.index.get() + 1);
        } else {
            panic!("Trying to advance token past end of stream")
        }
    }

    fn has_items(&self) -> bool {
        self.index.get() < self.tokens.borrow().len()
    }

    fn eat_token_if(&self, token: LexItem) {
        if *self.peek_current_token() == token {
            self.advance_stream();
        } else {
            panic!("Trying to advance the token stream, but got unexpected token.\n\
                    Got {:?} expexted {:?} ", self.peek_current_token(), token);
        }
    }


    pub fn parse(&self) -> Result<Selector, String> {
        let mut res = Selector {
            main_type: None,
            sub_type: None,
            identifier: None,
            child: None,
        };
        if self.has_items() {
            match *self.peek_current_token() {
                LexItem::Identifier(ref s) => {
                    self.advance_stream();
                    res.main_type = Some(s.to_string());
                }
                _ => {}
            }
        }
        if self.has_items() {
            match *self.peek_current_token() {
                LexItem::OpenSquare => {
                    self.eat_token_if(LexItem::OpenSquare);
                    let ident = match *self.get_current_token() {
                        LexItem::Identifier(ref s) => { s.to_string() }
                        _ => panic!("didnt find identifier inside square brackets")
                    };
                    self.eat_token_if(LexItem::CloseSquare);
                    res.sub_type = Some(ident);
                }
                _ => {}
            }
        }
        if self.has_items() {
            match *self.peek_current_token() {
                LexItem::Dot => {
                    self.eat_token_if(LexItem::Dot);
                    let ident = match *self.get_current_token() {
                        LexItem::Identifier(ref s) => { s.to_string() }
                        _ => panic!("didnt find identifier after dot")
                    };
                    res.identifier = Some(ident);
                }
                _ => {}
            }
        }
        Ok(res)
    }
}

fn lex_selector(selector: &str) -> Vec<LexItem> {
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
 enum LexItem {
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
    use select::SelectorParser;
    use select::lex_selector;
    use std::cell::RefCell;
    use std::cell::Cell;
    use lex::Lexer;
    use parse::Parser;
    use select::{select_entity,select_field};

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

    #[test]
    fn parse_test() {
        let s = "main[subType].identifier";
        let tokens = lex_selector(s);
        let parser = SelectorParser {
            tokens: RefCell::from(tokens),
            index: Cell::from(0),
        };

        let sel = parser.parse().unwrap();
        assert_eq!(sel.main_type.unwrap(), "main");
        assert_eq!(sel.sub_type.unwrap(), "subType");
        assert_eq!(sel.identifier.unwrap(), "identifier");
    }

    #[test]
    fn parse_test_just_main() {
        let s = "main";
        let tokens = lex_selector(s);
        let parser = SelectorParser {
            tokens: RefCell::from(tokens),
            index: Cell::from(0),
        };

        let sel = parser.parse().unwrap();
        assert_eq!(sel.main_type.unwrap(), "main");
        assert_eq!(sel.sub_type.is_none(), true);
        assert_eq!(sel.identifier.is_none(), true);
    }

    #[test]
    fn parse_test_just_sub_type() {
        let s = "[subtype]";
        let tokens = lex_selector(s);
        let parser = SelectorParser {
            tokens: RefCell::from(tokens),
            index: Cell::from(0),
        };

        let sel = parser.parse().unwrap();
        assert_eq!(sel.main_type.is_none(), true);
        assert_eq!(sel.sub_type.unwrap(), "subtype");
        assert_eq!(sel.identifier.is_none(), true);
    }

    #[test]
    fn parse_test_just_identifier() {
        let s = ".identifier";
        let tokens = lex_selector(s);
        let parser = SelectorParser {
            tokens: RefCell::from(tokens),
            index: Cell::from(0),
        };

        let sel = parser.parse().unwrap();
        assert_eq!(sel.main_type.is_none(), true);
        assert_eq!(sel.sub_type.is_none(), true);
        assert_eq!(sel.identifier.unwrap(), "identifier");
    }


    #[test]
    fn parse_test_sub_and_identifier() {
        let s = "[sub].identifier";
        let tokens = lex_selector(s);
        let parser = SelectorParser {
            tokens: RefCell::from(tokens),
            index: Cell::from(0),
        };
        let sel = parser.parse().unwrap();
        assert_eq!(sel.main_type.is_none(), true);
        assert_eq!(sel.sub_type.unwrap(), "sub");
        assert_eq!(sel.identifier.unwrap(), "identifier");
    }

    #[test]
    fn parse_test_main_and_identifier() {
        let s = "main.identifier";
        let tokens = lex_selector(s);
        let parser = SelectorParser {
            tokens: RefCell::from(tokens),
            index: Cell::from(0),
        };
        let sel = parser.parse().unwrap();
        assert_eq!(sel.main_type.unwrap(), "main");
        assert_eq!(sel.sub_type.is_none(), true);
        assert_eq!(sel.identifier.unwrap(), "identifier");
    }

    #[test]
    fn select_entity_simple() {
        let cdl = "
widget kpi {
    label : \"Label\"
    labels : \"Labels\"
}

widget kpi2 {
    label : \"Label\"
    labels : \"Labels\"
}
".to_string();
        let lexer = Lexer::new(cdl);
        let lex_items = lexer.lex().unwrap();
        let parser = Parser::new(lex_items);
        let root = parser.parse().unwrap();

        let result = select_entity(&root, "widget[kpi]");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn select_entity_simple2() {
        let cdl = "
page {

    widget kpi {
        label : \"Label\"
        labels : \"Labels\"
    }
}

page {

    widget kpi {
        label : \"Label\"
        labels : \"Labels\"
    }
    widget kpi2 {
        label : \"Label\"
        labels : \"Labels\"
    }
    widget kpi3 #kpiid {
        label : \"Label\"
        labels : \"Labels\"
    }
}
".to_string();
        let lexer = Lexer::new(cdl);
        let lex_items = lexer.lex().unwrap();
        let parser = Parser::new(lex_items);
        let root = parser.parse().unwrap();

        assert_eq!(select_entity(&root, "widget[kpi]").len(), 2);
        assert_eq!(select_entity(&root, "widget[kpi2]").len(), 1);
        assert_eq!(select_entity(&root, "widget").len(), 4);
        assert_eq!(select_entity(&root, "widget.kpiid").len(), 1);
    }

    #[test]
    fn select_field_simple() {
        let cdl = "
page {

    widget kpi {
        label : \"Label\"
        labels : \"Labels\"
    }
}

page {

    widget kpi {
        label : \"Label\"
        labels : \"Labels\"
    }
    widget kpi2 {
        label : \"Label\"
        labels : \"Labels\"
    }
    widget kpi3 #kpiid {
        label : \"Label\"
        labels : \"Labels\"
    }
}
".to_string();
        let lexer = Lexer::new(cdl);
        let lex_items = lexer.lex().unwrap();
        let parser = Parser::new(lex_items);
        let root = parser.parse().unwrap();

        assert_eq!(select_field(&root, ".label").len(), 4);
//        assert_eq!(select_entity(&root, "widget[kpi2]").len(), 1);
//        assert_eq!(select_entity(&root, "widget").len(), 4);
//        assert_eq!(select_entity(&root, "widget.kpiid").len(), 1);
    }


}


