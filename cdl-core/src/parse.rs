use lex::TokenStream;
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



pub fn parse(tokens : &mut TokenStream) -> Result<AstRootNode, String> {
    let mut root =AstRootNode{
        children: Vec::new(),
    };

    while  tokens.has_items() {
        match tokens.get_current_token() {
            LexItem::EOL => {
                tokens.advance_stream();
            },
            LexItem::Identifier(_) => {
                let entity  = parse_entity(tokens);
                root.children.push(entity);
            }
            _ => panic!("Error when parsing top level")
        }

    }
    Ok(root)
}

fn parse_entity (tokens: &mut TokenStream) -> AstEntityNode {
    let mut node = AstEntityNode {
        fields: Vec::new(),
        children: Vec::new(),
        identifier: String::new(),
        main_type: String::new(),
        sub_type: String::new(),
    };
    {
        let main_type = tokens.get_current_token();
        match main_type {
            LexItem::Identifier(m) => node.main_type = m.to_string(),
            _ => panic!("Trying to parse Entity, didnt find main type")
        }

    }
    tokens.advance_stream();
    let found_sub_type = {
        let sub_type = tokens.get_current_token();
        match sub_type {
            LexItem::Identifier(s) => {
                node.sub_type = s.to_string();
                true
            }
            _ => {false}
        }

    };
    if found_sub_type {
        tokens.advance_stream();
    }

    let body = parse_entity_body(tokens);
    node.fields = body.0;
    node.children = body.1;
    node
}

fn parse_entity_body(tokens : &mut TokenStream) -> (Vec<AstFieldNode>, Vec<AstEntityNode> ) {
    assert_eq!(tokens.get_current_token() , &LexItem::OpenBracket);
    tokens.advance_stream();
    assert_eq!(tokens.get_current_token() , &LexItem::EOL);
    tokens.advance_stream();
    let mut fields = Vec::new();
    let mut entities = Vec::new();

    loop {
        let is_done = match tokens.get_current_token() {
          LexItem::CloseBracket => true,
          _ => false
        };

        if is_done {
            tokens.advance_stream(); // close brace
            tokens.advance_stream(); // EOL
            return (fields, entities);
        }

        let next_is_field = match ( tokens.get_current_token(), tokens.get_next_token()) {
            (LexItem::Identifier(_), LexItem::Identifier(_)) => false,
            (LexItem::Identifier(_), LexItem::Colon) => true,
            (_,_) => panic!("Trying to parse entitiy body, and not field or entity found")
        };
        if next_is_field {
            fields.push(parse_field(tokens));

        } else {
            entities.push(parse_entity(tokens))
        }
    }

    //return (fields, entities);
}

fn parse_field (tokens: &mut TokenStream) ->AstFieldNode {
    let mut node = AstFieldNode {
        identifier: String::new(),
        value: RHS::String("".to_string())
    };
    {
        let identifier = tokens.get_current_token();
        match identifier {
            LexItem::Identifier(m) => node.identifier = m.to_string(),
            _ => panic!("Didnt find field name {:?}", identifier)
        }
    }

    tokens.advance_stream();
    tokens.advance_stream(); // colon
    // parse RHS
    {
        let rhs = tokens.get_current_token();
        match rhs {
            LexItem::String(m) => node.value =RHS::String(m.to_string()),
            _ => panic!("Didnt find rhs ")
        }
    }
    tokens.advance_stream();
    tokens.advance_stream(); // EOL
    node
}

