use std::cell::RefCell;
use std::cell::Cell;
use select::lex::LexItem;
use std::cell::Ref;

#[derive(Debug)]
pub struct Selector {
    pub main_type: Option<String>,
    pub sub_type: Option<String>,
    pub identifier: Option<String>,
    pub child: Option<Box<Selector>>,
}

#[derive(Debug)]
pub struct SelectorParser {
    tokens: RefCell<Vec<LexItem>>,
    index: Cell<usize>,
}

impl SelectorParser {
    pub fn new(tokens: Vec<LexItem>) -> SelectorParser {
        SelectorParser {
            tokens: RefCell::from(tokens),
            index: Cell::from(0),
        }
    }

    fn peek_current_token(&self) -> Ref<LexItem> {
        Ref::map(self.tokens.borrow(), |tokens| &tokens[self.index.get()])
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
        return Ok(self.parse_selector()?);
    }


    fn parse_selector(&self) -> Result<Selector,String> {
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
                LexItem::Arrow => {
                    self.advance_stream();
                    res.child = Some(Box::new( self.parse_selector()?));
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
                LexItem::Arrow => {
                    self.advance_stream();
                    res.child = Some(Box::new( self.parse_selector()?));
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
                LexItem::Arrow => {
                    self.advance_stream();
                    res.child = Some(Box::new( self.parse_selector()?));
                }
                _ => {}
            }
        }
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use select::parse::SelectorParser;
    use select::lex::lex_selector;

    #[test]
    fn parse_test() {
        let s = "main[subType].identifier";
        let tokens = lex_selector(s);
        let parser = SelectorParser::new(tokens);

        let sel = parser.parse().unwrap();
        assert_eq!(sel.main_type.unwrap(), "main");
        assert_eq!(sel.sub_type.unwrap(), "subType");
        assert_eq!(sel.identifier.unwrap(), "identifier");
    }

    #[test]
    fn parse_test_just_main() {
        let s = "main";
        let tokens = lex_selector(s);
        let parser = SelectorParser::new(tokens);

        let sel = parser.parse().unwrap();
        assert_eq!(sel.main_type.unwrap(), "main");
        assert_eq!(sel.sub_type.is_none(), true);
        assert_eq!(sel.identifier.is_none(), true);
    }

    #[test]
    fn parse_test_just_sub_type() {
        let s = "[subtype]";
        let tokens = lex_selector(s);
        let parser = SelectorParser::new(tokens);

        let sel = parser.parse().unwrap();
        assert_eq!(sel.main_type.is_none(), true);
        assert_eq!(sel.sub_type.unwrap(), "subtype");
        assert_eq!(sel.identifier.is_none(), true);
    }

    #[test]
    fn parse_test_just_identifier() {
        let s = ".identifier";
        let tokens = lex_selector(s);
        let parser = SelectorParser::new(tokens);

        let sel = parser.parse().unwrap();
        assert_eq!(sel.main_type.is_none(), true);
        assert_eq!(sel.sub_type.is_none(), true);
        assert_eq!(sel.identifier.unwrap(), "identifier");
    }


    #[test]
    fn parse_test_sub_and_identifier() {
        let s = "[sub].identifier";
        let tokens = lex_selector(s);
        let parser = SelectorParser::new(tokens);
        let sel = parser.parse().unwrap();
        assert_eq!(sel.main_type.is_none(), true);
        assert_eq!(sel.sub_type.unwrap(), "sub");
        assert_eq!(sel.identifier.unwrap(), "identifier");
    }

    #[test]
    fn parse_test_main_and_identifier() {
        let s = "main.identifier";
        let tokens = lex_selector(s);
        let parser = SelectorParser::new(tokens);
        let sel = parser.parse().unwrap();
        assert_eq!(sel.main_type.unwrap(), "main");
        assert_eq!(sel.sub_type.is_none(), true);
        assert_eq!(sel.identifier.unwrap(), "identifier");
    }

    #[test]
    fn parse_sub_selectors() {
        let s = "main[kpi] > [kpi].label";
        let tokens = lex_selector(s);
        let parser = SelectorParser::new(tokens);
        let sel = parser.parse().unwrap();
        assert_eq!(sel.main_type.unwrap(), "main");
        assert_eq!(sel.sub_type.unwrap(), "kpi");
        assert_eq!(sel.identifier.is_none(), true);
        assert_eq!(sel.child.is_some(), true);
        let child = sel.child.unwrap();
        assert_eq!(child.identifier.unwrap(), "label");
    }
}


