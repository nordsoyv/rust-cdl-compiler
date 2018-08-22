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
    let cdl = "widget kpi {\
    label : \"Label\"\
}".to_string();
    let res = lex(&cdl);
    let lex_items =res.unwrap();
    assert_eq!(lex_items.get_length(), 7);
    println!("{:?}", lex_items)
}

#[test]
fn simple_parse(){
    let cdl = "widget kpi {\
    label : \"Label\"\
    label2 : \"Label2\"\
}".to_string();
    let mut lex_items = lex(&cdl).unwrap();
    let root = parse(&mut lex_items);
    println!("{:?}", root.unwrap())

}