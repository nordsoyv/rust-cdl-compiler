mod lex;
mod parse;

use parse::AstRootNode;
use parse::Parser;
use lex::Lexer;


pub fn compile(cdl : String) -> Result<AstRootNode , String> {
    let lexer = Lexer::new(cdl);
    let lex_items = lexer.lex().unwrap();
    let mut parser = Parser::new(lex_items);
    let root = parser.parse();
    root
}

#[test]
fn simple_lex() {
    let cdl = "widget kpi {
    label : \"Label\"
}".to_string();
    let lexer = Lexer::new(cdl);
    let res = lexer.lex();
    let lex_items =res.unwrap();
    assert_eq!(lex_items.len(), 9);
}

#[test]
fn lex_reference() {
    let cdl = "widget kpi @default {
    label : \"Label\"
}".to_string();
    let lexer = Lexer::new(cdl);
    let res = lexer.lex();
    let lex_items =res.unwrap();
    assert_eq!(lex_items.len(), 10);
}


#[test]
fn parse_entity(){
    let cdl = "widget kpi {
    label : \"Label\"
    labels : \"Labels\"
}
".to_string();
    let lexer = Lexer::new(cdl);
    let lex_items = lexer.lex().unwrap();
    let mut parser = Parser::new(lex_items);
    let root = parser.parse().unwrap();
    assert_eq!(root.children.len(), 1);
    assert_eq!(root.children[0].body.fields.len(), 2);
}

#[test]
fn parse_2_entity(){
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
    let mut parser = Parser::new(lex_items);
    let root = parser.parse().unwrap();
    assert_eq!(root.children.len(), 2);
    assert_eq!(root.children[0].body.fields.len(), 2);
    assert_eq!(root.children[1].body.fields.len(), 2);
}

#[test]
fn entity_with_no_subtype(){
    let cdl = "
widget   {
    label : \"Label\"
    labels : \"Labels\"
}
".to_string();
    let lexer = Lexer::new(cdl);
    let lex_items = lexer.lex().unwrap();
    let mut parser = Parser::new(lex_items);
    let root = parser.parse().unwrap();
    assert_eq!(root.children.len(), 1);
    assert_eq!(root.children[0].body.fields.len(), 2);
    assert_eq!(root.children[0].sub_type, "");
}

#[test]
fn entity_with_entity_inside_entity(){
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
    let mut parser = Parser::new(lex_items);
    let root = parser.parse().unwrap();
    assert_eq!(root.children.len(), 1);
    assert_eq!(root.children[0].body.fields.len(), 1);
    assert_eq!(root.children[0].body.children.len(), 1);
    assert_eq!(root.children[0].body.children[0].body.fields.len(), 1);
}

#[test]
fn parse_entity_with_id(){
    let cdl = "id: widget kpi {
    label : \"Label\"
    labels : \"Labels\"
}
".to_string();
    let lexer = Lexer::new(cdl);
    let lex_items = lexer.lex().unwrap();
    let mut parser = Parser::new(lex_items);
    let root = parser.parse().unwrap();
    assert_eq!(root.children.len(), 1);
    assert_eq!(root.children[0].identifier, "id".to_string());
    assert_eq!(root.children[0].body.fields.len(), 2);
}

#[test]
fn parse_entity_with_reference(){
    let cdl = "id: widget kpi @default {
    label : \"Label\"
    labels : \"Labels\"
}
".to_string();
    let lexer = Lexer::new(cdl);
    let lex_items = lexer.lex().unwrap();
    let mut parser = Parser::new(lex_items);
    let root = parser.parse().unwrap();
    assert_eq!(root.children.len(), 1);
    assert_eq!(root.children[0].identifier, "id".to_string());
    assert_eq!(root.children[0].reference, "default".to_string());
    assert_eq!(root.children[0].body.fields.len(), 2);
}

