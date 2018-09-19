use std::cell::{Cell, Ref, RefCell};
use lex::LexItem;

#[derive(Debug)]
pub enum Expr {
    String(Box<AstStringNode>),
    Identifier(Box<AstIdentifierNode>),
    Number(Box<AstNumberNode>),
    Function(Box<AstFunctionNode>),
    VPath(Box<AstVPathNode>),
    Operator(Box<AstOperatorNode>),
    UnaryOperator(Box<AstUnaryOperatorNode>),
}

#[derive(Debug)]
pub struct AstStringNode {
    pub value: String
}

#[derive(Debug)]
pub struct AstIdentifierNode {
    pub value: String
}

#[derive(Debug)]
pub struct AstNumberNode {
    pub value: f64,
    pub text_rep: String,
}

impl AstNumberNode {
    pub fn new(number: f64, text_rep: String) -> AstNumberNode {
        AstNumberNode {
            value: number,
            text_rep,
        }
    }
}

#[derive(Debug)]
pub struct AstFunctionNode {
    pub identifier: String,
    pub argument_list: Vec<EntityExprRef>,
}

#[derive(Debug)]
pub struct AstOperatorNode {
    pub operator: char,
    pub left_side: EntityExprRef,
    pub right_side: EntityExprRef,
}

#[derive(Debug)]
pub struct AstUnaryOperatorNode {
    pub operator: char,
    pub expr: EntityExprRef,
}


#[derive(Debug)]
pub struct AstVPathNode {
    pub table: Option<String>,
    pub sub_table: Option<String>,
    pub field: Option<String>,
    pub sub_field: Option<String>,
}

#[derive(Debug)]
pub struct AstRootNode {
    pub children: Vec<EntityRef>,
}

#[derive(Debug)]
pub struct AstEntityNode {
    pub main_type: String,
    pub sub_type: Option<String>,
    pub reference: Option<String>,
    pub identifier: Option<String>,
    pub fields: Vec<EntityFieldRef>,
    pub children: Vec<EntityRef>,

}

impl AstEntityNode {
    fn new() -> AstEntityNode {
        AstEntityNode {
            main_type: String::new(),
            sub_type: None,
            reference: None,
            identifier: None,
            fields: Vec::new(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct AstFieldNode {
    pub identifier: String,
    pub value: EntityExprRef,
}


type EntityRef = usize;
type EntityFieldRef = usize;
type EntityExprRef = usize;


#[derive(Debug)]
pub struct ParseResult {
    pub root: AstRootNode,
    entities: Vec<AstEntityNode>,
    fields: Vec<AstFieldNode>,
    expressions: Vec<Expr>,
}

impl ParseResult {
    pub fn get_entity(&self, r: EntityRef) -> &AstEntityNode {
        &self.entities[r]
    }

    pub fn add_entity(&mut self, node: AstEntityNode) -> EntityRef {
        self.entities.push(node);
        self.entities.len() - 1
    }

    pub fn get_field(&self, r: EntityFieldRef) -> &AstFieldNode {
        &self.fields[r]
    }

    pub fn add_field(&mut self, node: AstFieldNode) -> EntityFieldRef {
        self.fields.push(node);
        self.fields.len() - 1
    }

    pub fn get_expr(&self, r: EntityExprRef) -> &Expr {
        &self.expressions[r]
    }

    pub fn add_expr(&mut self, node: Expr) -> EntityExprRef {
        self.expressions.push(node);
        self.expressions.len() - 1
    }
}


#[derive(Debug)]
pub struct Parser {
    tokens: RefCell<Vec<LexItem>>,
    index: Cell<usize>,
}


impl Parser {
    pub fn new(tokens: Vec<LexItem>) -> Parser {
        Parser {
            tokens: RefCell::new(tokens),
            index: Cell::new(0),
        }
    }

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

    pub fn parse(&self) -> Result<ParseResult, String> {
        let mut root = AstRootNode {
            children: Vec::new(),
        };
        let mut pr = ParseResult {
            root: AstRootNode {
                children: Vec::new(),
            },
            entities: Vec::new(),
            fields: Vec::new(),
            expressions: Vec::new(),
        };
        while self.has_items() {
            match *self.peek_current_token() {
                LexItem::EOL => {
                    self.advance_stream();
                }
                LexItem::Identifier(_) => {
                    let index = self.parse_entity(&mut pr)?;
                    root.children.push(index);
                }
                _ => { return Err(format!("Error when parsing top level, found {:?}", self.peek_current_token())); }
            }
        }
        pr.root = root;
        Ok(pr)
    }

    fn parse_entity(&self, pr: &mut ParseResult) -> Result<EntityRef, String> {
        let mut node = AstEntityNode::new();
        match *self.get_current_token() {
            LexItem::Identifier(ref m) => node.main_type = m.to_string(),
            ref token @ _ => return Err(format!("Trying to parse Entity, didnt find main type. Found {:?} instead", token))
        }

        match self.get_entity_subtype() {
            Some(s) => {
                node.sub_type = Some(s);
                self.advance_stream()
            }
            None => {}
        }

        match self.get_entity_id() {
            Some(s) => {
                node.identifier = Some(s);
                self.advance_stream()
            }
            None => {}
        }

        match self.get_entity_reference() {
            Some(s) => {
                node.reference = Some(s);
                self.advance_stream();
            }
            None => {}
        }
        self.eat_token_if(LexItem::OpenBracket);
        self.eat_token_if(LexItem::EOL);
        let mut fields = Vec::new();
        let mut entities = Vec::new();

        loop {
            // are we done?
            match *self.peek_current_token() {
                LexItem::CloseBracket => {
                    self.eat_token_if(LexItem::CloseBracket);
                    self.eat_token_if(LexItem::EOL);
                    break;
                }
                _ => {}
            };
            // skip blank lines
            match *self.peek_current_token() {
                LexItem::EOL => {
                    self.eat_token_if(LexItem::EOL);
                    continue;
                }
                _ => {}
            }

            // try parsing next line
            match (&*self.peek_current_token(), &*self.peek_next_token()?) {
                (LexItem::Identifier(_), LexItem::Colon) => fields.push(self.parse_field(pr)?),
                (LexItem::Identifier(_), _) => entities.push(self.parse_entity(pr)?),
                (_, _) => return Err("Trying to parse entity body, and not field or entity found".to_string())
            }
        }
        node.children = entities;
        node.fields = fields;
        let index = pr.add_entity(node);
        Ok(index)
    }

    fn get_entity_subtype(&self) -> Option<String> {
        match *self.peek_current_token() {
            LexItem::Identifier(ref s) => Some(s.to_string()),
            _ => None
        }
    }

    fn get_entity_reference(&self) -> Option<String> {
        match *self.peek_current_token() {
            LexItem::Reference(ref s) => Some(s.to_string()),
            _ => None
        }
    }

    fn get_entity_id(&self) -> Option<String> {
        match *self.peek_current_token() {
            LexItem::Identifier(ref s) => Some(s.to_string()),
            _ => None
        }
    }

    fn parse_field(&self, pr: &mut ParseResult) -> Result<EntityFieldRef, String> {
        let mut node = AstFieldNode {
            identifier: String::new(),
            value: 0,
        };

        match *self.get_current_token() {
            LexItem::Identifier(ref m) => node.identifier = m.to_string(),
            ref identifier @ _ => return Err(format!("Didnt find field identifier, instead got {:?}", identifier))
        }

        self.eat_token_if(LexItem::Colon);
        node.value = self.parse_expr(pr)?;
        let index = pr.add_field(node);
        self.eat_token_if(LexItem::EOL);
        Ok(index)
    }


    // E --> T {( "+" | "-" ) T}
    fn parse_expr(&self, pr: &mut ParseResult) -> Result<EntityExprRef, String> {
        let mut current_expr = self.parse_term(pr)?;
        loop {
            match *self.peek_current_token() {
                LexItem::Minus => {
                    self.advance_stream();
                    let right_side = self.parse_term(pr)?;
                    let index = pr.add_expr(Expr::Operator(Box::new(AstOperatorNode {
                        operator: '-',
                        left_side: current_expr,
                        right_side,
                    })));
                    current_expr = index;
                }
                LexItem::Plus => {
                    self.advance_stream();
                    let right_side = self.parse_term(pr)?;
                    let index = pr.add_expr(Expr::Operator(Box::new(AstOperatorNode {
                        operator: '+',
                        left_side: current_expr,
                        right_side,
                    })));
                    current_expr = index;
                }
                LexItem::EOL => {
                    return Ok(current_expr);
                }
                _ => {
                    return Ok(current_expr);
                }
//                ref t @ _ => return Err(format!("Found unexpected token when trying to parse expression: {:?}", t))
            }
        }
    }

    // T --> F {( "*" | "/" ) F}
    fn parse_term(&self, pr: &mut ParseResult) -> Result<EntityExprRef, String> {
        let mut current_expr = self.parse_factor(pr)?;
//        println!("Current term : {:?}", current_expr);
        loop {
            match *self.peek_current_token() {
                LexItem::Mul => {
                    self.advance_stream();
                    let right_side = self.parse_factor(pr)?;
                    let index = pr.add_expr(Expr::Operator(Box::new(AstOperatorNode {
                        operator: '*',
                        left_side: current_expr,
                        right_side,
                    })));
                    current_expr = index;
                }
                LexItem::Div => {
                    self.advance_stream();
                    let right_side = self.parse_factor(pr)?;
                    let index = pr.add_expr(Expr::Operator(Box::new(AstOperatorNode {
                        operator: '/',
                        left_side: current_expr,
                        right_side,
                    })));
                    current_expr = index;
                }
                _ => {
                    return Ok(current_expr);
                }
//                t @ _ => return Err(format!("Found unexpected token when trying to parse term: {:?}", t))
            }
        }
    }


    // F --> v | "(" E ")" | "-" T
    fn parse_factor(&self, pr: &mut ParseResult) -> Result<EntityExprRef, String> {
        match *self.peek_current_token() {
            LexItem::Number { ref value, ref real_text } => {
                self.advance_stream();
                let index = pr.add_expr(Expr::Number(Box::new(AstNumberNode {
                    value: *value,
                    text_rep: real_text.to_string(),
                })));
                return Ok(index);
            }
            LexItem::String(ref s) => {
                self.advance_stream();
                let index = pr.add_expr(Expr::String(Box::new(AstStringNode {
                    value: s.to_string(),
                })));
                return Ok(index);
            }
            LexItem::Identifier(ref s) => {
                match *self.peek_next_token()? {
                    LexItem::Colon => {
                        let path = self.parse_vpath(pr)?;
                        return Ok(path);
                    }
                    LexItem::OpenPar => {
                        let path = self.parse_function(pr)?;
                        return Ok(path);
                    }
                    _ => {
                        self.advance_stream();
                        let index = pr.add_expr(Expr::Identifier(Box::new(AstIdentifierNode {
                            value: s.to_string(),
                        })));
                        return Ok(index);
                    }
                }
            }
            LexItem::OpenPar => {
                self.advance_stream();
                let expr = self.parse_expr(pr)?;
                self.eat_token_if(LexItem::ClosePar);
                return Ok(expr);
            }
            LexItem::Minus => {
                self.advance_stream();
                let term = self.parse_term(pr)?;
                let index = pr.add_expr(Expr::UnaryOperator(Box::new(AstUnaryOperatorNode {
                    operator: '-',
                    expr: term,
                })));
                return Ok(index);
            }
            ref t @ _ => return Err(format!("Found unexpected token when trying to parse factor: {:?}", t))
        }
    }

    fn parse_vpath(&self, pr: &mut ParseResult) -> Result<EntityExprRef, String> {
        let source = match *self.get_current_token() {
            LexItem::Identifier(ref s) => {
                s.to_string()
            }
            ref t @ _ => return Err(format!("Found unexpected token when trying to parse vpath: {:?}", t))
        };
        self.eat_token_if(LexItem::Colon);
        let question = match *self.get_current_token() {
            LexItem::Identifier(ref s) => {
                s.to_string()
            }
            ref t @ _ => return Err(format!("Found unexpected token when trying to parse vpath: {:?}", t))
        };
        let index = pr.add_expr(Expr::VPath(Box::new(AstVPathNode {
            table: Some(source),
            sub_table: None,
            field: Some(question),
            sub_field: None,
        })));
        return Ok(index);
    }

    fn parse_function(&self, pr: &mut ParseResult) -> Result<EntityExprRef, String> {
        let name = match *self.get_current_token() {
            LexItem::Identifier(ref s) => {
                s.to_string()
            }
            ref t @ _ => return Err(format!("Found unexpected token when trying to parse function: {:?}", t))
        };
        self.eat_token_if(LexItem::OpenPar);
        let arg_list = self.parse_arg_list(pr)?;
        self.eat_token_if(LexItem::ClosePar);
        let index = pr.add_expr(Expr::Function(Box::new(AstFunctionNode {
            identifier: name,
            argument_list: arg_list,
        })));
        return Ok(index);
    }

    fn parse_arg_list(&self, pr: &mut ParseResult) -> Result<Vec<EntityExprRef>, String> {
        let mut args = Vec::new();
        loop {
            match *self.peek_current_token() {
                LexItem::Comma => {
                    self.advance_stream();
                }
                LexItem::ClosePar => {
                    return Ok(args);
                }
                _ => {
                    args.push(self.parse_expr(pr)?);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use lex::Lexer;
    use parse::Parser;

    const EXPR_CDL: &str = "widget kpi   {
    expr1: 1 + 1
    expr1: 1 * 1
    expr1: 1 * -1
    expr1: 1 - 1
    expr1: 1 + 1 + 1 + 1
    expr1: 1 + (1 + 1) + 1
    expr1: s1
    expr1: s1:q1
    expr1: NPS(s1:q1)
    expr1: NPS(s1:q1, MAX(1 , 2 ,3))
}
";

    #[test]
    fn parse_entity() {
        let cdl = "widget kpi {
    expr : 1 + 2
    id : identifier
    label : \"Label\"
    number: 1234
}
".to_string();
        let lexer = Lexer::new(cdl);
        let lex_items = lexer.lex().unwrap();
        let parser = Parser::new(lex_items);
        let pr = parser.parse().unwrap();
//        println!("{:?}", pr);
        assert_eq!(pr.fields.len(), 4);
        assert_eq!(pr.entities.len(), 1);
        assert_eq!(pr.expressions.len(), 6);
    }
        #[test]
        fn parse_2_entity() {
            let cdl = "
    widget kpi {
        label : \"Label\"
        labels : \"Labels\"
    }

    widget kpi {
        label : \"Label\"
        labels : \"Labels\"
    }
    ".to_string();
            let lexer = Lexer::new(cdl);
            let lex_items = lexer.lex().unwrap();
            let parser = Parser::new(lex_items);
            let pr = parser.parse().unwrap();
            assert_eq!(pr.entities.len(), 2);
            assert_eq!(pr.entities[0].fields.len(), 2);
            assert_eq!(pr.entities[1].fields.len(), 2);
        }

          #[test]
          fn parse_script_from_js() {
              let cdl = "
         datatable kpi data1 {
            type : nps
            vpath : t1:q1
          }

          page #overview {
            widget kpi kpi1{
              type : nps
              vpath : t1:q1
              label : \"KPI\"
            }
            widget kpi kpi2{
              type : nps
              vpath : t1:q1
              label : \"KPI\"
            }

            widget account {
              type : nps
              vpath : t1:q1
              label : \"KPI\"
            }
          }
      ".to_string();
              let lexer = Lexer::new(cdl);
              let lex_items = lexer.lex().unwrap();
              let parser = Parser::new(lex_items);
              let pr = parser.parse().unwrap();
              assert_eq!(pr.entities.len(), 5);
              assert_eq!(pr.entities[0].fields.len(), 2);
              assert_eq!(pr.entities[4].children.len(), 3);
          }

          #[test]
          fn entity_with_no_subtype() {
              let cdl = "
      widget   {
          label : \"Label\"
          labels : \"Labels\"
      }
      ".to_string();
              let lexer = Lexer::new(cdl);
              let lex_items = lexer.lex().unwrap();
              let parser = Parser::new(lex_items);
              let pr = parser.parse().unwrap();
              assert_eq!(pr.entities.len(), 1);
              assert_eq!(pr.entities[0].fields.len(), 2);
              assert_eq!(pr.entities[0].sub_type, None);
          }

          #[test]
          fn entity_with_entity_inside_entity() {
              let cdl = "
      widget kpi  {
          label : \"Label\"

          tile kpi {
             type : \"type\"
          }
      }
      ".to_string();
              let lexer = Lexer::new(cdl);
              let lex_items = lexer.lex().unwrap();
              let parser = Parser::new(lex_items);
              let pr = parser.parse().unwrap();
              assert_eq!(pr.entities.len(), 2);
              assert_eq!(pr.entities[0].fields.len(), 1);
              assert_eq!(pr.entities[0].children.len(), 0);
              assert_eq!(pr.entities[1].fields.len(), 1);
              assert_eq!(pr.entities[1].children.len(), 1);
          }

          #[test]
          fn parse_entity_with_id() {
              let cdl = " widget kpi #id {
          label : \"Label\"
          labels : \"Labels\"
      }
      ".to_string();
              let lexer = Lexer::new(cdl);
              let lex_items = lexer.lex().unwrap();
              let parser = Parser::new(lex_items);
              let pr = parser.parse().unwrap();
              assert_eq!(pr.entities.len(), 1);
              assert_eq!(pr.entities[0].identifier, Some("id".to_string()));
              assert_eq!(pr.entities[0].fields.len(), 2);
          }

          #[test]
          fn parse_entity_with_reference() {
              let cdl = "widget kpi  #id @default {
          label : \"Label\"
          labels : \"Labels\"
      }
      ".to_string();
              let lexer = Lexer::new(cdl);
              let lex_items = lexer.lex().unwrap();
              let parser = Parser::new(lex_items);
              let pr = parser.parse().unwrap();
              assert_eq!(pr.entities.len(), 1);
              assert_eq!(pr.entities[0].identifier, Some("id".to_string()));
              assert_eq!(pr.entities[0].reference, Some("default".to_string()));
              assert_eq!(pr.entities[0].fields.len(), 2);
          }

          #[test]
          fn parse_entity_with_expr() {
              let lexer = Lexer::new(EXPR_CDL.to_string());
              let lex_items = lexer.lex().unwrap();
              let parser = Parser::new(lex_items);
              let pr = parser.parse().unwrap();
              assert_eq!(pr.entities.len(), 1);
              assert_eq!(pr.entities[0].fields.len(), 10);
          }
}

