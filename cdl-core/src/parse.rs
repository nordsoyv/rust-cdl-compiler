use lex::TokenStream;
use lex::LexItem;

#[derive(Debug)]
pub enum RHS {
    String(String)
}

#[derive(Debug)]
pub struct AstRootNode {
    children : Vec<AstEntityNode>,
}

#[derive(Debug)]
struct AstEntityNode {
    fields : Vec<AstFieldNode>,
    children : Vec<AstEntityNode>,
    identifier : String,
    main_type : String,
    sub_type : String
}

#[derive(Debug)]
struct AstFieldNode {
    identifier : String,
    value : RHS,
}



pub fn parse(tokens : &mut TokenStream) -> Result<AstRootNode, String> {
    let mut root =AstRootNode{
        children: Vec::new(),
    };

    while  tokens.has_items() {
        let entity  = parse_entity(tokens);
        root.children.push(entity);
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
            _ => panic!("Didnt find main type")
        }

    }
    tokens.advance_stream();
    {
        let sub_type = tokens.get_current_token();
        println!("sub type: {:?}", sub_type);
        match sub_type {
            LexItem::Identifier(s) => node.sub_type = s.to_string(),
            _ => panic!("Didnt find sub type")
        }
    }
    tokens.advance_stream();
    // open brace
    tokens.advance_stream();

    let fields = parse_fields(tokens);
    node.fields = fields;
    node
}

fn parse_fields ( tokens: &mut TokenStream) ->Vec<AstFieldNode> {
    let mut fields = Vec::new();
    loop {
        let field = parse_field(tokens);
        fields.push(field);
        {
            if *tokens.get_current_token() == LexItem::CloseBracket {
                tokens.advance_stream();
                return fields;
            }
        }
    }
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
            _ => panic!("Didnt find field name")
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
    node
}

