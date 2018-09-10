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
    pub argument_list: Vec<Expr>,
}

#[derive(Debug)]
pub struct AstOperatorNode {
    pub operator: char,
    pub left_side: Expr,
    pub right_side: Expr,
}

#[derive(Debug)]
pub struct AstUnaryOperatorNode {
    pub operator: char,
    pub expr: Expr,
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
    pub children: Vec<AstEntityNode>,
}

#[derive(Debug)]
pub struct AstEntityNode {
    pub header: AstEntityHeaderNode,
    pub body: AstEntityBodyNode,
}

impl AstEntityNode {
    fn new() -> AstEntityNode {
        AstEntityNode {
            body: AstEntityBodyNode::new(),
            header: AstEntityHeaderNode::new(),
        }
    }
}

#[derive(Debug)]
pub struct AstEntityBodyNode {
    pub fields: Vec<AstFieldNode>,
    pub children: Vec<AstEntityNode>,
}

impl AstEntityBodyNode {
    fn new() -> AstEntityBodyNode {
        AstEntityBodyNode {
            fields: Vec::new(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct AstEntityHeaderNode {
    pub main_type: String,
    pub sub_type: Option<String>,
    pub reference: Option<String>,
    pub identifier: Option<String>,
}

impl AstEntityHeaderNode {
    fn new() -> AstEntityHeaderNode {
        AstEntityHeaderNode {
            main_type: String::new(),
            sub_type: None,
            reference: None,
            identifier: None,
        }
    }
}

#[derive(Debug)]
pub struct AstFieldNode {
    pub identifier: String,
    pub value: Expr,
}

#[derive(Debug)]
pub struct Parser {
    tokens: RefCell<Vec<LexItem>>,
    index: Cell<usize>,
}

#[allow(dead_code)]
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

    pub fn parse(&self) -> Result<AstRootNode, String> {
        let mut root = AstRootNode {
            children: Vec::new(),
        };

        while self.has_items() {
            match *self.peek_current_token() {
                LexItem::EOL => {
                    self.advance_stream();
                }
                LexItem::Identifier(_) => {
                    let entity = self.parse_entity()?;
                    root.children.push(entity);
                }
                _ => { return Err(format!("Error when parsing top level, found {:?}", self.peek_current_token())); }
            }
        }
        Ok(root)
    }

    fn parse_entity(&self) -> Result<AstEntityNode, String> {
        let mut node = AstEntityNode::new();
        node.header = self.parse_entity_header()?;
        node.body = self.parse_entity_body()?;
        Ok(node)
    }

    fn parse_entity_header(&self) -> Result<AstEntityHeaderNode, String> {
        let mut node = AstEntityHeaderNode::new();
        // get main type
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
        Ok(node)
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

    fn parse_entity_body(&self) -> Result<AstEntityBodyNode, String> {
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
                    return Ok(AstEntityBodyNode {
                        fields,
                        children: entities,
                    });
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
                (LexItem::Identifier(_), LexItem::Colon) => fields.push(self.parse_field()?),
                (LexItem::Identifier(_), _) => entities.push(self.parse_entity()?),
                (_, _) => return Err("Trying to parse entity body, and not field or entity found".to_string())
            }
        }
    }

    fn parse_field(&self) -> Result<AstFieldNode, String> {
        let mut node = AstFieldNode {
            identifier: String::new(),
            value: Expr::String(Box::new(AstStringNode { value: String::new() })),
        };

        match *self.get_current_token() {
            LexItem::Identifier(ref m) => node.identifier = m.to_string(),
            ref identifier @ _ => return Err(format!("Didnt find field identifier, instead got {:?}", identifier))
        }

        self.eat_token_if(LexItem::Colon);
        node.value = self.parse_expr()?;
        self.eat_token_if(LexItem::EOL);
        Ok(node)
    }


    // E --> T {( "+" | "-" ) T}
    fn parse_expr(&self) -> Result<Expr, String> {
        let mut current_expr = self.parse_term()?;
        loop {
            match *self.peek_current_token() {
                LexItem::Minus => {
                    self.advance_stream();
                    let right_side = self.parse_term()?;
                    current_expr = Expr::Operator(Box::new(AstOperatorNode {
                        operator: '-',
                        left_side: current_expr,
                        right_side,
                    }));
                }
                LexItem::Plus => {
                    self.advance_stream();
                    let right_side = self.parse_term()?;
                    current_expr = Expr::Operator(Box::new(AstOperatorNode {
                        operator: '+',
                        left_side: current_expr,
                        right_side,
                    }));
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
    fn parse_term(&self) -> Result<Expr, String> {
        let mut current_expr = self.parse_factor()?;
//        println!("Current term : {:?}", current_expr);
        loop {
            match *self.peek_current_token() {
                LexItem::Mul => {
                    self.advance_stream();
                    let right_side = self.parse_factor()?;
                    current_expr = Expr::Operator(Box::new(AstOperatorNode {
                        operator: '*',
                        left_side: current_expr,
                        right_side,
                    }));
                }
                LexItem::Div => {
                    self.advance_stream();
                    let right_side = self.parse_factor()?;
                    current_expr = Expr::Operator(Box::new(AstOperatorNode {
                        operator: '/',
                        left_side: current_expr,
                        right_side,
                    }));
                }
                _ => {
                    return Ok(current_expr);
                }
//                t @ _ => return Err(format!("Found unexpected token when trying to parse term: {:?}", t))
            }
        }
    }


    // F --> v | "(" E ")" | "-" T
    fn parse_factor(&self) -> Result<Expr, String> {
        match *self.peek_current_token() {
            LexItem::Number { ref value, ref real_text } => {
                self.advance_stream();
                return Ok(Expr::Number(Box::new(AstNumberNode {
                    value: *value,
                    text_rep: real_text.to_string(),
                })));
            }
            LexItem::String(ref s) => {
                self.advance_stream();
                return Ok(Expr::String(Box::new(AstStringNode {
                    value: s.to_string(),
                })));
            }
            LexItem::Identifier(ref s) => {
                match *self.peek_next_token()? {
                    LexItem::Colon => {
                        let path = self.parse_vpath()?;
                        return Ok(path);
                    }
                    LexItem::OpenPar => {
                        let path = self.parse_function()?;
                        return Ok(path);
                    }
                    _ => {
                        self.advance_stream();
                        return Ok(Expr::Identifier(Box::new(AstIdentifierNode {
                            value: s.to_string(),
                        })));
                    }
                }
            }
            LexItem::OpenPar => {
                self.advance_stream();
                let expr = self.parse_expr()?;
                self.eat_token_if(LexItem::ClosePar);
                return Ok(expr);
            }
            LexItem::Minus => {
                self.advance_stream();
                let term = self.parse_term()?;
                return Ok(Expr::UnaryOperator(Box::new(AstUnaryOperatorNode {
                    operator: '-',
                    expr: term,
                })));
            }
            ref t @ _ => return Err(format!("Found unexpected token when trying to parse factor: {:?}", t))
        }
    }

    fn parse_vpath(&self) -> Result<Expr, String> {
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

        return Ok(Expr::VPath(Box::new(AstVPathNode {
            table: Some(source),
            sub_table: None,
            field: Some(question),
            sub_field: None,
        })));
    }

    fn parse_function(&self) -> Result<Expr, String> {
        let name = match *self.get_current_token() {
            LexItem::Identifier(ref s) => {
                s.to_string()
            }
            ref t @ _ => return Err(format!("Found unexpected token when trying to parse function: {:?}", t))
        };
        self.eat_token_if(LexItem::OpenPar);
        let arg_list = self.parse_arg_list()?;
        self.eat_token_if(LexItem::ClosePar);
        return Ok(Expr::Function(Box::new(AstFunctionNode {
            identifier: name,
            argument_list: arg_list,
        })));
    }

    fn parse_arg_list(&self) -> Result<Vec<Expr>, String> {
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
                    args.push(self.parse_expr()?);
                }

            }
        }
    }
}