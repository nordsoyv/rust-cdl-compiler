use lex::LexItem;

#[derive(Debug)]
pub enum Expr {
    String(Box<AstStringNode>),
    Identifier(Box<AstIdentifierNode>),
    Number(Box<AstNumberNode>),
    Function(Box<AstFunctionNode>),
    VPath(Box<AstVPathNode>),
    Operator(Box<AstOperatorNode>),
}

#[derive(Debug)]
pub struct AstStringNode {
    pub value : String
}

#[derive(Debug)]
pub struct AstIdentifierNode {
    pub value : String
}

#[derive(Debug)]
pub struct AstNumberNode {
    pub value : f64,
    pub text_rep : String
}

impl AstNumberNode {
    pub fn new(number : f64, text_rep : String) -> AstNumberNode {
        AstNumberNode {
            value : number,
            text_rep,
        }
    }
}

#[derive(Debug)]
pub struct AstFunctionNode {
    pub identifier : String,
    pub argument_list : Vec<Expr>
}

#[derive(Debug)]
pub struct AstOperatorNode {
    pub operator : char,
    pub left_side: Expr,
    pub right_side :Expr
}

#[derive(Debug)]
pub struct AstVPathNode {
    pub table: Option<String>,
    pub sub_table : Option<String>,
    pub field : Option<String>,
    pub sub_field : Option<String>,
}

#[derive(Debug)]
pub struct AstRootNode {
    pub children: Vec<AstEntityNode>,
}

#[derive(Debug)]
pub struct AstEntityNode {
    pub header : AstEntityHeaderNode,
    pub body: AstEntityBodyNode,
}

impl AstEntityNode {
    fn new() -> AstEntityNode {
        AstEntityNode {
            body: AstEntityBodyNode::new(),
            header : AstEntityHeaderNode::new(),
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
            main_type : String::new(),
            sub_type : None,
            reference : None,
            identifier : None,
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
    tokens: Vec<LexItem>,
    index: usize,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<LexItem>) -> Parser {
        Parser {
            tokens,
            index: 0,
        }
    }

    fn get_length(&self) -> usize {
        self.tokens.len()
    }

    fn peek_current_token(&self) -> &LexItem {
        &self.tokens[self.index]
    }

    fn peek_next_token(&self) -> Result<&LexItem, String> {
        if self.index + 1 < self.tokens.len() {
            return Ok(&self.tokens[self.index + 1]);
        }
        Err(format!("Trying to access token past end of stream"))
    }

    fn get_current_token(&mut self) -> &LexItem {
        self.advance_stream();
        &self.tokens[self.index - 1]
    }

    fn advance_stream(&mut self) {
        if self.index + 1 <= self.tokens.len() {
            self.index += 1;
        } else {
            panic!("Trying to advance token past end of stream")
        }
    }

    fn has_items(&self) -> bool {
        self.index < self.tokens.len()
    }

    fn eat_token_if(&mut self, token: LexItem) {
        if *self.peek_current_token() == token {
            self.advance_stream();
        } else {
            panic!("Trying to advance the token stream, but got unexpected token.\n\
                    Got {:?} expexted {:?} ", self.peek_current_token(), token);
        }
    }

    pub fn parse(&mut self) -> Result<AstRootNode, String> {
        let mut root = AstRootNode {
            children: Vec::new(),
        };

        while self.has_items() {
            match self.peek_current_token() {
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

    fn parse_entity(&mut self) -> Result<AstEntityNode, String> {
        let mut node = AstEntityNode::new();
        node.header = self.parse_entity_header()?;
        node.body = self.parse_entity_body()?;
        Ok(node)
    }

    fn parse_entity_header(&mut self) -> Result<AstEntityHeaderNode, String> {
        let mut node = AstEntityHeaderNode::new();
        match (self.peek_current_token(), self.peek_next_token()?) {
            (LexItem::Identifier(_), LexItem::Colon) => {
                match self.get_current_token() {
                    LexItem::Identifier(ident) => node.identifier = Some(ident.to_string()),
                    id @ _ => return Err(format!("Trying to get identifier for entity, found {:?}", id))
                }
                self.eat_token_if(LexItem::Colon);
            }
            _ => {}
        };
        // get main type
        match self.get_current_token() {
            LexItem::Identifier(m) => node.main_type = m.to_string(),
            token @ _ => return Err(format!("Trying to parse Entity, didnt find main type. Found {:?} instead", token))
        }

        match self.get_entity_subtype() {
            Some(s) => {
                node.sub_type = Some(s);
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

    fn get_entity_subtype(&mut self) -> Option<String> {
        match self.peek_current_token() {
            LexItem::Identifier(s) => Some(s.to_string()),
            _ => None
        }
    }

    fn get_entity_reference(&mut self) -> Option<String> {
        match self.peek_current_token() {
            LexItem::Reference(s) => Some(s.to_string()),
            _ => None
        }
    }

    fn parse_entity_body(&mut self) -> Result<AstEntityBodyNode, String> {
        self.eat_token_if(LexItem::OpenBracket);
        self.eat_token_if(LexItem::EOL);
        let mut fields = Vec::new();
        let mut entities = Vec::new();

        loop {
            // are we done?
            match self.peek_current_token() {
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
            match self.peek_current_token() {
                LexItem::EOL => {
                    self.eat_token_if(LexItem::EOL);
                    continue;
                }
                _ => {}
            }

            // try parsing next line
            match (self.peek_current_token(), self.peek_next_token()?) {
                (LexItem::Identifier(_), LexItem::Colon) => fields.push(self.parse_field()?),
                (LexItem::Identifier(_), _) => entities.push(self.parse_entity()?),
                (_, _) => return Err("Trying to parse entity body, and not field or entity found".to_string())
            }
        }
    }


    fn parse_field(&mut self) -> Result<AstFieldNode, String> {
        let mut node = AstFieldNode {
            identifier: String::new(),
            value: Expr::String(Box::new( AstStringNode { value: String::new() })),
        };

        match self.get_current_token() {
            LexItem::Identifier(m) => node.identifier = m.to_string(),
            identifier @ _ => return Err(format!("Didnt find field identifier, instead got {:?}", identifier))
        }

        self.eat_token_if(LexItem::Colon);
        node.value = self.parse_rhs()?;
        self.eat_token_if(LexItem::EOL);
        Ok(node)
    }


    // E --> T {( "+" | "-" ) T}
    fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut term = self.parse_term()?;
        match self.peek_current_token() {
            LexItem::Minus | LexItem::Plus => {
                           }
            LexItem::EOL => {
                return Ok(term);
            }
            t @_ => return Err(format!("Unexpcted token when trying to parse epxr: {:?}", t));
        }
    }

    // T --> F {( "*" | "/" ) F}
    fn parse_term(&mut self) -> Result<Expr, String> {

    }
    // F --> v | "(" E ")" | "-" T
    fn parse_factor(&mut self) -> Result<Expr, String> {

    }

    fn parse_rhs(&mut self) -> Result<Expr,String> {
        match self.get_current_token() {
            LexItem::String(m) => {
                return Ok(Expr::String(Box::new( AstStringNode { value: m.to_string() })));
            } ,
            LexItem::Identifier(i) => {
                return Ok(Expr::Identifier(Box::new(AstIdentifierNode{value: i.to_string()})));
            }
            LexItem::Number{ value , real_text } => {
                return Ok(Expr::Number(Box::new(AstNumberNode{value : *value, text_rep : real_text.to_string() })));
            }
            _ => return Err(format!("Didnt find rhs "))
        }
    }
}




/*
Recursive descent parsing an expression
 E --> T {( "+" | "-" ) T}
 T --> F {( "*" | "/" ) F}
 F --> v | "(" E ")" | "-" T

v =  value


*/
