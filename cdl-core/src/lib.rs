mod lex;
mod parse;
mod print;
mod select;

use parse::AstRootNode;
use parse::Parser;
use lex::Lexer;
pub use select::{select_field, select_entity};

pub fn compile(cdl: String) -> Result<AstRootNode, String> {
    let lexer = Lexer::new(cdl);
    let lex_items = lexer.lex().unwrap();
    let parser = Parser::new(lex_items);
    let root = parser.parse();
    root
}

pub fn print(root: AstRootNode) -> String {
    print::print(root)
}



#[cfg(test)]
mod test {
    use lex::Lexer;
    use parse::Parser;
    use print;

    const EXPR_CDL: &str = "widget kpi   {
    expr1: 1 + 1
    expr1: 1 * 1
    expr1: 1 * -1
    expr1: 1 - 1
    expr1: 1 + 1 + 1 + 1
    expr1: 1 + (1 + 1) + 1
    expr1: s1
    expr1: s1:q1
    expr1: NPS(s1:q1)
    expr1: NPS(s1:q1, MAX(1 , 2 ,3))
}
";

    #[test]
    fn simple_lex() {
        let cdl = "widget kpi {
    label : \"Label\"
}".to_string();
        let lexer = Lexer::new(cdl);
        let res = lexer.lex();
        let lex_items = res.unwrap();
        assert_eq!(lex_items.len(), 9);
    }

    #[test]
    fn lex_reference() {
        let cdl = "widget kpi @default {
    label : \"Label\"
}".to_string();
        let lexer = Lexer::new(cdl);
        let res = lexer.lex();
        let lex_items = res.unwrap();
        assert_eq!(lex_items.len(), 10);
    }

    #[test]
    fn lex_id() {
        let cdl = "widget kpi #id {
    label : \"Label\"
}".to_string();
        let lexer = Lexer::new(cdl);
        let res = lexer.lex();
        let lex_items = res.unwrap();
        assert_eq!(lex_items.len(), 10);
    }

    #[test]
    fn lex_extended() {
        let cdl = "widget kpi @default {
    label : a(b+c)
}".to_string();
        let lexer = Lexer::new(cdl);
        let res = lexer.lex();
        let lex_items = res.unwrap();
        assert_eq!(lex_items.len(), 15);
    }


    #[test]
    fn lex_advanced_expr(){
        let cdl = "value: MAX(survey:Q2,survey:interview_start=max(survey:interview_start))
        value: average(score(survey:Q7), @cr.currentPeriodB2b)
        thresholds: #82D854 >= 100%, #FFBD5B >= 80%, #FA5263 < 80%
        riskValue: IIF(average(SCORE(survey:Q1))<7,'H!',IIF(average(SCORE(survey:Q1))>8,'L',IIF(COUNT(survey:responseid)<1,'U','M')))".to_string();
        let lexer = Lexer::new(cdl);
        let res = lexer.lex();
        let lex_items = res.unwrap();
        println!("{:?}", lex_items);
    }
    #[test]
    fn parse_entity() {
        let cdl = "widget kpi {
    expr : 1 + 2
    id : identifier
    label : \"Label\"
    number: 1234
}
".to_string();
        let lexer = Lexer::new(cdl);
        let lex_items = lexer.lex().unwrap();
        let parser = Parser::new(lex_items);
        let root = parser.parse().unwrap();
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].body.fields.len(), 4);
        //println!("{:?}", root.children[0].body.fields[2]);
    }

    #[test]
    fn parse_2_entity() {
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
        let parser = Parser::new(lex_items);
        let root = parser.parse().unwrap();
        assert_eq!(root.children.len(), 2);
        assert_eq!(root.children[0].body.fields.len(), 2);
        assert_eq!(root.children[1].body.fields.len(), 2);
    }

    #[test]
    fn parse_script_from_js() {
        let cdl = "
   datatable kpi data1 {
      type : nps
      vpath : t1:q1
    }

    page #overview {
      widget kpi kpi1{
        type : nps
        vpath : t1:q1
        label : \"KPI\"
      }
      widget kpi kpi2{
        type : nps
        vpath : t1:q1
        label : \"KPI\"
      }

      widget account {
        type : nps
        vpath : t1:q1
        label : \"KPI\"
      }
    }
".to_string();
        let lexer = Lexer::new(cdl);
        let lex_items = lexer.lex().unwrap();
        let parser = Parser::new(lex_items);
        let root = parser.parse().unwrap();
        assert_eq!(root.children.len(), 2);
        assert_eq!(root.children[0].body.fields.len(), 2);
        assert_eq!(root.children[1].body.children.len(), 3);
    }

    #[test]
    fn entity_with_no_subtype() {
        let cdl = "
widget   {
    label : \"Label\"
    labels : \"Labels\"
}
".to_string();
        let lexer = Lexer::new(cdl);
        let lex_items = lexer.lex().unwrap();
        let parser = Parser::new(lex_items);
        let root = parser.parse().unwrap();
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].body.fields.len(), 2);
        assert_eq!(root.children[0].header.sub_type, None);
    }

    #[test]
    fn entity_with_entity_inside_entity() {
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
        let parser = Parser::new(lex_items);
        let root = parser.parse().unwrap();
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].body.fields.len(), 1);
        assert_eq!(root.children[0].body.children.len(), 1);
        assert_eq!(root.children[0].body.children[0].body.fields.len(), 1);
    }

    #[test]
    fn parse_entity_with_id() {
        let cdl = " widget kpi #id {
    label : \"Label\"
    labels : \"Labels\"
}
".to_string();
        let lexer = Lexer::new(cdl);
        let lex_items = lexer.lex().unwrap();
        let parser = Parser::new(lex_items);
        let root = parser.parse().unwrap();
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].header.identifier, Some("id".to_string()));
        assert_eq!(root.children[0].body.fields.len(), 2);
    }

    #[test]
    fn parse_entity_with_reference() {
        let cdl = "widget kpi  #id @default {
    label : \"Label\"
    labels : \"Labels\"
}
".to_string();
        let lexer = Lexer::new(cdl);
        let lex_items = lexer.lex().unwrap();
        let parser = Parser::new(lex_items);
        let root = parser.parse().unwrap();
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].header.identifier, Some("id".to_string()));
        assert_eq!(root.children[0].header.reference, Some("default".to_string()));
        assert_eq!(root.children[0].body.fields.len(), 2);
    }

    #[test]
    fn parse_entity_with_expr() {
        let lexer = Lexer::new(EXPR_CDL.to_string());
        let lex_items = lexer.lex().unwrap();
        let parser = Parser::new(lex_items);
        let root = parser.parse().unwrap();
        assert_eq!(root.children.len(), 1);
        assert_eq!(root.children[0].body.fields.len(), 10);
    }

    #[test]
    fn print_cdl_expr_cdl() {
        let lexer = Lexer::new(EXPR_CDL.to_string());
        let lex_items = lexer.lex().unwrap();
        let parser = Parser::new(lex_items);
        let root = parser.parse().unwrap();
        let out = print::print(root);
        let correct = "widget kpi {
    expr1: 1 + 1
    expr1: 1 * 1
    expr1: 1 * -1
    expr1: 1 - 1
    expr1: 1 + 1 + 1 + 1
    expr1: 1 + 1 + 1 + 1
    expr1: s1
    expr1: s1:q1
    expr1: NPS(s1:q1)
    expr1: NPS(s1:q1, MAX(1, 2, 3))
}
".to_string();
        assert_eq!(out, correct);
    }
}

#[test]
fn print_cdl() {
    let cdl = "widget kpi #id @default {
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
    let parser = Parser::new(lex_items);
    let root = parser.parse().unwrap();
    let out = print::print(root);
    let correct = "widget kpi #id @default {
    label: \"Label\"
    id: identifier
    number: 1234.001000
    tile kpi {
        type: \"type\"
    }
}
".to_string();
    assert_eq!(out, correct);
}


#[test]
fn print_expressions() {
    let cdl = "widget kpi {
    expr : 1 + 1
    expr : 1 * 1
    expr : 1 * -1
}
".to_string();
    let lexer = Lexer::new(cdl);
    let lex_items = lexer.lex().unwrap();
    let parser = Parser::new(lex_items);
    let root = parser.parse().unwrap();
    let out = print::print(root);
    let correct = "widget kpi {
    expr: 1 + 1
    expr: 1 * 1
    expr: 1 * -1
}
".to_string();
    assert_eq!(out, correct);
}



