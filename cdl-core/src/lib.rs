mod lex;
mod parse;
mod print;

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

pub fn print(root : AstRootNode) -> String {
    print::print(root)
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
fn lex_extended() {
    let cdl = "widget kpi @default {
    label : a(b+c)
}".to_string();
    let lexer = Lexer::new(cdl);
    let res = lexer.lex();
    let lex_items =res.unwrap();
    assert_eq!(lex_items.len(), 15);
}


#[test]
fn parse_entity(){
    let cdl = "widget kpi {
    label : \"Label\"
    id : identifier
    number: 1234
}
".to_string();
    let lexer = Lexer::new(cdl);
    let lex_items = lexer.lex().unwrap();
    let mut parser = Parser::new(lex_items);
    let root = parser.parse().unwrap();
    assert_eq!(root.children.len(), 1);
    assert_eq!(root.children[0].body.fields.len(), 3);
    //println!("{:?}", root.children[0].body.fields[2]);
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
    assert_eq!(root.children[0].header.sub_type, None);
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
    assert_eq!(root.children[0].header.identifier, Some("id".to_string()));
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
    assert_eq!(root.children[0].header.identifier, Some("id".to_string()));
    assert_eq!(root.children[0].header.reference, Some("default".to_string()));
    assert_eq!(root.children[0].body.fields.len(), 2);
}

#[test]
fn print_cdl(){
    let cdl = "id: widget kpi @default {
    label : \"Label\"
    id : identifier
    number : 1234.001000
    tile kpi {
        type : \"type\"
    }
}
".to_string();
    let lexer = Lexer::new(cdl);
    let lex_items = lexer.lex().unwrap();
    let mut parser = Parser::new(lex_items);
    let root = parser.parse().unwrap();
    let out = print::print(root);
    let correct = "id: widget kpi @default {
    label: \"Label\"
    id: identifier
    number: 1234.001000
    tile kpi {
        type: \"type\"
    }
}
".to_string();
    assert_eq!(out,correct);
}

