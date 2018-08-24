use lex::LexItem;

#[derive(Debug)]
pub enum RHS {
    String(String)
}

#[derive(Debug)]
pub struct AstRootNode {
    pub children: Vec<AstEntityNode>,
}

#[derive(Debug)]
pub struct AstEntityNode {
    pub main_type: String,
    pub sub_type: String,
    pub reference : String,
    pub identifier: String,
    pub body: AstEntityBodyNode,
}

impl AstEntityNode {
    fn new() -> AstEntityNode {
        AstEntityNode {
            body: AstEntityBodyNode::new(),
            identifier: String::new(),
            main_type: String::new(),
            sub_type: String::new(),
            reference: String::new(),
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
pub struct AstFieldNode {
    pub identifier: String,
    pub value: RHS,
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
         match (self.peek_current_token(), self.peek_next_token()?) {
            (LexItem::Identifier(_), LexItem::Colon) => {
                match self.get_current_token() {
                    LexItem::Identifier(ident) => node.identifier = ident.to_string(),
                    id @ _ => return Err(format!("Trying to get identifier for entity, found {:?}", id))
                }
                self.eat_token_if(LexItem::Colon);
            }
            _ => {}
        };
        match self.get_current_token() {
            LexItem::Identifier(m) => node.main_type = m.to_string(),
            token @ _ => return Err(format!("Trying to parse Entity, didnt find main type. Found {:?} instead", token))
        }

        let found_sub_type = {
            let sub_type = self.peek_current_token();
            match sub_type {
                LexItem::Identifier(s) => {
                    node.sub_type = s.to_string();
                    true
                }
                _ => { false }
            }
        };
        if found_sub_type {
            self.advance_stream();
        }

        let found_reference = {
            let reference = self.peek_current_token();
            match reference {
                LexItem::Reference(s) => {
                    node.reference = s.to_string();
                    true
                }
                _ => { false }
            }
        };
        if found_reference {
            self.advance_stream();
        }

        let body = self.parse_entity_body()?;
        node.body = body;
        Ok(node)
    }


    fn parse_entity_body(&mut self) -> Result<AstEntityBodyNode, String> {
        self.eat_token_if(LexItem::OpenBracket);
        self.eat_token_if(LexItem::EOL);
        let mut fields = Vec::new();
        let mut entities = Vec::new();

        loop {
            let is_done = match self.peek_current_token() {
                LexItem::CloseBracket => true,
                _ => false
            };

            if is_done {
                self.eat_token_if(LexItem::CloseBracket);
                self.eat_token_if(LexItem::EOL);
                return Ok(AstEntityBodyNode {
                    fields,
                    children: entities,
                });
            }

            match self.peek_current_token() {
                LexItem::EOL => {
                    self.eat_token_if(LexItem::EOL);
                    continue;
                }
                _ => {}
            }

            let next_is_field = match (self.peek_current_token(), self.peek_next_token()?) {
                (LexItem::Identifier(_), LexItem::Identifier(_)) => false,
                (LexItem::Identifier(_), LexItem::Colon) => true,
                (_, _) => return Err("Trying to parse entitiy body, and not field or entity found".to_string())
            };
            if next_is_field {
                fields.push(self.parse_field()?);
            } else {
                entities.push(self.parse_entity()?)
            }
        }
    }


    fn parse_field(&mut self) -> Result<AstFieldNode, String> {
        let mut node = AstFieldNode {
            identifier: String::new(),
            value: RHS::String("".to_string()),
        };
        {
            let identifier = self.peek_current_token();
            match identifier {
                LexItem::Identifier(m) => node.identifier = m.to_string(),
                _ => return Err(format!("Didnt find field name {:?}", identifier))
            }
        }

        self.advance_stream();
        self.eat_token_if(LexItem::Colon);
        // parse RHS
        {
            let rhs = self.peek_current_token();
            match rhs {
                LexItem::String(m) => node.value = RHS::String(m.to_string()),
                _ => return Err(format!("Didnt find rhs "))
            }
        }
        self.advance_stream();
        self.eat_token_if(LexItem::EOL);
        Ok(node)
    }
}




