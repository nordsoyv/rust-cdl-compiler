use lex::lex;
use parse::AstRootNode;
use parse::Parser;

mod lex;
mod parse;

pub fn compile(cdl : String) -> Result<AstRootNode , String> {
    let  lex_items = lex(&cdl).unwrap();
    let mut parser = Parser::new(lex_items);
    let root = parser.parse();
    root
}

#[test]
fn simple_lex() {
    let cdl = "widget kpi {
    label : \"Label\"
}".to_string();
    let res = lex(&cdl);
    let lex_items =res.unwrap();
    assert_eq!(lex_items.len(), 9);
}

#[test]
fn parse_entity(){
    let cdl = "widget kpi {
    label : \"Label\"
    labels : \"Labels\"
}
".to_string();
    let lex_items = lex(&cdl).unwrap();
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
    let lex_items = lex(&cdl).unwrap();
    let mut parser = Parser::new(lex_items);
    let root = parser.parse().unwrap();
  //  println!("{:?}", root);
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
    let  lex_items = lex(&cdl).unwrap();
    let mut parser = Parser::new(lex_items);
    let root = parser.parse().unwrap();
    //println!("{:?}", root.unwrap())
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
    let  lex_items = lex(&cdl).unwrap();
    let mut parser = Parser::new(lex_items);
    let root = parser.parse().unwrap();
    assert_eq!(root.children.len(), 1);
    assert_eq!(root.children[0].body.fields.len(), 1);
    assert_eq!(root.children[0].body.children.len(), 1);
    assert_eq!(root.children[0].body.children[0].body.fields.len(), 1);
}