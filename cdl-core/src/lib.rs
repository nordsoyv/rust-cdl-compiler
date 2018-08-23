use lex::lex;
use parse::parse;
use parse::AstRootNode;

mod lex;
mod parse;

pub fn compile(cdl : String) -> Result<AstRootNode , String> {
    let mut lex_items = lex(&cdl).unwrap();
    let root = parse(&mut lex_items);
    root
}

#[test]
fn simple_lex() {
    let cdl = "widget kpi {
    label : \"Label\"
}".to_string();
    let res = lex(&cdl);
    let lex_items =res.unwrap();
    assert_eq!(lex_items.get_length(), 9);
//    println!("{:?}", lex_items)
}

#[test]
fn simple_parse(){
    let cdl = "widget kpi {
    label : \"Label\"
    labels : \"Labels\"
}".to_string();
    let mut lex_items = lex(&cdl).unwrap();
//    println!("{:?}", lex_items);
    let root = parse(&mut lex_items).unwrap();
//    println!("{:?}", root);
    assert_eq!(root.children.len(), 1);
    assert_eq!(root.children[0].fields.len(), 2);
}

#[test]
fn simple_parse2(){
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
    let mut lex_items = lex(&cdl).unwrap();
    let root = parse(&mut lex_items).unwrap();
//    println!("{:?}", root);
    assert_eq!(root.children.len(), 2);
    assert_eq!(root.children[0].fields.len(), 2);
    assert_eq!(root.children[1].fields.len(), 2);
}

#[test]
fn parse_with_no_subtype(){
    let cdl = "
widget   {
    label : \"Label\"
    labels : \"Labels\"
}
".to_string();
    let mut lex_items = lex(&cdl).unwrap();
    let _root = parse(&mut lex_items);
    //println!("{:?}", root.unwrap())
}

#[test]
fn parse_with_entity_inside_entity(){
    let cdl = "
widget kpi  {
    label : \"Label\"
    tile kpi {
       type : \"type\"
    }
}
".to_string();
    let mut lex_items = lex(&cdl).unwrap();
    let root = parse(&mut lex_items).unwrap();
    //println!("{:?}", root);
    assert_eq!(root.children.len(), 1);
    assert_eq!(root.children[0].fields.len(), 1);
    assert_eq!(root.children[0].children.len(), 1);
    assert_eq!(root.children[0].children[0].fields.len(), 1);
}