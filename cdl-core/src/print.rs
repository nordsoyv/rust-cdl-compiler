use parse::AstEntityNode;
use parse::AstFieldNode;
use parse::Expr;
use parse::ParseResult;
use std;

pub fn print(pr: ParseResult ) -> String {
    let mut res = String::new();
    for child_id in &pr.root.children {
        let child = pr.get_entity(*child_id);
        let child_str = print_entity(child, 0, &pr);
        res.push_str(&child_str);
    }
    return res;
}

fn print_entity(entity: &AstEntityNode, indent: usize,pr: &ParseResult ) -> String {
    let mut res = print_entity_header(&entity, indent);
    res += &print_entity_body(&entity, indent + 1, pr);
    return res;
}

fn print_entity_header(header: &AstEntityNode, indent: usize) -> String {
    let indent = create_indent(indent);
    let mut res = "".to_string();
    res.push_str(&indent);
    res.push_str(&header.main_type);
    res.push_str(" ");
    match header.sub_type {
        Some(ref id) => {
            res.push_str(&id);
            res.push_str(" ")
        }
        None => {}
    }

    match header.identifier {
        Some(ref id) => {
            res.push_str("#");
            res.push_str(&id);
            res.push_str(" ")
        }
        None => {}
    }

    match header.reference {
        Some(ref id) => {
            res.push_str("@");
            res.push_str(&id);
            res.push_str(" ")
        }
        None => {}
    }

    return res;
}

fn print_entity_body(body: &AstEntityNode, indent: usize, pr: &ParseResult) -> String {
    let mut res = "{\n".to_string();

    for field_id in &body.fields {
        let field = pr.get_field(*field_id);
        res.push_str(&print_field(field, indent + 1, pr));
    }

    for child_id in &body.children {
        let child = pr.get_entity(*child_id);
        res.push_str(&print_entity(child, indent + 1,pr));
    }
    res.push_str(&create_indent(indent - 1));
    res.push_str("}\n");
    return res;
}

fn print_field(field: &AstFieldNode, indent: usize, pr : &ParseResult) -> String {
    let mut res = "".to_string();

    res.push_str(&create_indent(indent));
    res.push_str(&field.identifier);
    res.push_str(": ");
    let expr = pr.get_expr(field.value);
    res.push_str(&print_expr(expr, pr));
    res.push_str("\n");
    res
}

fn print_expr(expr: &Expr, pr : &ParseResult) -> String {
    let mut res = "".to_string();

    match expr {
        Expr::Operator(node) => {
            let left_node = pr.get_expr(node.left_side);
            let right_node = pr.get_expr(node.right_side);
            let left_side = print_expr(left_node, pr);
            let right_side = print_expr(right_node,pr);
            res.push_str(&left_side);
            res.push_str(" ");
            res.push(node.operator);
            res.push_str(" ");
            res.push_str(&right_side);
        }
        Expr::Identifier(node) => {
            res.push_str(&node.value);
        }
        Expr::String(node) => {
            res.push_str("\"");
            res.push_str(&node.value);
            res.push_str("\"");
        }
        Expr::UnaryOperator(node) => {
            res.push(node.operator);
            let expr = pr.get_expr(node.expr);
            res.push_str(&print_expr(expr, pr));
        }
        Expr::Number(node) => {
            res.push_str(&node.text_rep);
        }
        Expr::Function(node) => {
            let mut arg_list = Vec::new();
            res.push_str(&node.identifier);
            res.push('(');
            for arg in &node.argument_list {
                let expr = pr.get_expr(*arg);
                arg_list.push(print_expr(expr, pr));
            }
            res.push_str(&(arg_list.join(", ")));
            res.push(')');
        }
        Expr::VPath(node) => {
            match node.table {
                Some(ref s) => {
                    res.push_str(s);
                }
                _ => {}
            }
            match node.sub_table {
                Some(ref s) => {
                    res.push_str(s);
                }
                _ => {}
            }

            res.push(':');
            match node.field {
                Some(ref s) => {
                    res.push_str(s);
                }
                _ => {}
            }

            match node.sub_field {
                Some(ref s) => {
                    res.push_str(s);
                }
                _ => {}
            }
        }
    }
    res
}

fn create_indent(indent: usize) -> String {
    std::iter::repeat(" ").take(indent * 2).collect::<String>()
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
}
