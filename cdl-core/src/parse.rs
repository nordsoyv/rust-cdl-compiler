use lex::LexItem;

#[derive(Debug)]
pub enum RHS {
    String(String)
}

#[derive(Debug)]
pub struct AstRootNode {
    pub children : Vec<AstEntityNode>,
}

#[derive(Debug)]
pub struct AstEntityNode {
    pub fields : Vec<AstFieldNode>,
    pub children : Vec<AstEntityNode>,
    pub identifier : String,
    pub main_type : String,
    pub sub_type : String
}

#[derive(Debug)]
pub struct AstFieldNode {
    pub identifier : String,
    pub value : RHS,
}

#[derive(Debug)]
pub struct Parser {
    tokens : Vec<LexItem>,
    index : usize,
}

#[allow(dead_code)]
impl Parser {
    pub fn new (tokens : Vec<LexItem>) -> Parser {
        Parser {
            tokens,
            index: 0,
        }
    }

    fn get_length(&self)-> usize {
        self.tokens.len()
    }

    fn get_current_token (&self) -> &LexItem{
        &self.tokens[self.index]
    }

    fn get_next_token  (&self) -> &LexItem {
        if self.index + 1 < self.tokens.len() {
            return &self.tokens[self.index+1]
        }
        panic!("Trying to access token past end of stream")
    }

    fn advance_stream(&mut self){
        self.index += 1;
    }

    fn has_items(&self) -> bool{
        self.index < self.tokens.len()
    }

    pub fn parse(&mut self) -> Result<AstRootNode, String> {
        let mut root =AstRootNode{
            children: Vec::new(),
        };

        while  self.has_items() {
            match self.get_current_token() {
                LexItem::EOL => {
                    self.advance_stream();
                },
                LexItem::Identifier(_) => {
                    let entity  = self.parse_entity();
                    root.children.push(entity);
                }
                _ => panic!("Error when parsing top level")
            }

        }
        Ok(root)
    }

    fn parse_entity (&mut self) -> AstEntityNode {
        let mut node = AstEntityNode {
            fields: Vec::new(),
            children: Vec::new(),
            identifier: String::new(),
            main_type: String::new(),
            sub_type: String::new(),
        };
        {
            let main_type = self.get_current_token();
            match main_type {
                LexItem::Identifier(m) => node.main_type = m.to_string(),
                _ => panic!("Trying to parse Entity, didnt find main type")
            }

        }
        self.advance_stream();
        let found_sub_type = {
            let sub_type = self.get_current_token();
            match sub_type {
                LexItem::Identifier(s) => {
                    node.sub_type = s.to_string();
                    true
                }
                _ => {false}
            }

        };
        if found_sub_type {
            self.advance_stream();
        }

        let body = self.parse_entity_body();
        node.fields = body.0;
        node.children = body.1;
        node
    }


    fn parse_entity_body(&mut self) -> (Vec<AstFieldNode>, Vec<AstEntityNode> ) {
        assert_eq!(self.get_current_token() , &LexItem::OpenBracket);
        self.advance_stream();
        assert_eq!(self.get_current_token() , &LexItem::EOL);
        self.advance_stream();
        let mut fields = Vec::new();
        let mut entities = Vec::new();

        loop {
            let is_done = match self.get_current_token() {
                LexItem::CloseBracket => true,
                _ => false
            };

            if is_done {
                self.advance_stream(); // close brace
                self.advance_stream(); // EOL
                return (fields, entities);
            }

            let next_is_field = match ( self.get_current_token(), self.get_next_token()) {
                (LexItem::Identifier(_), LexItem::Identifier(_)) => false,
                (LexItem::Identifier(_), LexItem::Colon) => true,
                (_,_) => panic!("Trying to parse entitiy body, and not field or entity found")
            };
            if next_is_field {
                fields.push(self.parse_field());

            } else {
                entities.push(self.parse_entity())
            }
        }

        //return (fields, entities);
    }


    fn parse_field (&mut self) ->AstFieldNode {
        let mut node = AstFieldNode {
            identifier: String::new(),
            value: RHS::String("".to_string())
        };
        {
            let identifier = self.get_current_token();
            match identifier {
                LexItem::Identifier(m) => node.identifier = m.to_string(),
                _ => panic!("Didnt find field name {:?}", identifier)
            }
        }

        self.advance_stream();
        self.advance_stream(); // colon
        // parse RHS
        {
            let rhs = self.get_current_token();
            match rhs {
                LexItem::String(m) => node.value =RHS::String(m.to_string()),
                _ => panic!("Didnt find rhs ")
            }
        }
        self.advance_stream();
        self.advance_stream(); // EOL
        node
    }



}




