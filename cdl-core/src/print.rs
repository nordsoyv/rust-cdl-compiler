use parse::AstRootNode;
use parse::AstEntityNode;
use parse::AstEntityHeaderNode;
use parse::AstEntityBodyNode;
use parse::AstFieldNode;
use parse::Expr;
use std;

pub fn print(root: AstRootNode) -> String {
    let mut res = String::new();
    for child in &root.children {
        let child_str = print_entity(child, 0);
        res.push_str(&child_str);
    }
    return res;
}

fn print_entity(entity: &AstEntityNode, indent: usize) -> String {
    let mut res = print_entity_header(&entity.header, indent);
    res += &print_entity_body(&entity.body, indent + 1);
    return res;
}

fn print_entity_header(header: &AstEntityHeaderNode, indent: usize) -> String {
    let indent = create_indent(indent);
    let mut res = "".to_string();
    res.push_str(&indent);
    match header.identifier {
        Some(ref id) => {
            res.push_str(&id);
            res.push_str(": ")
        }
        None => {}
    }
    res.push_str(&header.main_type);
    res.push_str(" ");
    match header.sub_type {
        Some(ref id) => {
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

fn print_entity_body(body: &AstEntityBodyNode, indent: usize) -> String {
    let mut res = "{\n".to_string();

    for field in &body.fields {
        res.push_str(&print_field(&field, indent + 1));
    }

    for child in &body.children {
        res.push_str(&print_entity(&child, indent + 1));
    }
    res.push_str(&create_indent(indent - 1));
    res.push_str("}\n");
    return res;
}

fn print_field(field: &AstFieldNode, indent: usize) -> String {
    let mut res = "".to_string();

    res.push_str(&create_indent(indent));
    res.push_str(&field.identifier);
    res.push_str(": ");
    match field.value {
        Expr::String(ref s) => {
            res.push_str("\"");
            res.push_str(&s.value);
            res.push_str("\"");
        }
        Expr::Identifier(ref s) => {
            res.push_str(&s.value);
        }
        Expr::Number(ref n) => {
            res.push_str(&n.text_rep.to_string());
        }
        Expr::Function(_) => panic!("Trying to print function"),
        Expr::VPath(_) => panic!("Trying to print VPath"),
        Expr::Operator(_) => panic!("Trying to print Operator"),
        Expr::UnaryOperator(_) => panic!("Trying to print UnaryOperator"),
    }
    res.push_str("\n");
    res
}

fn create_indent(indent: usize) -> String {
    std::iter::repeat(" ").take(indent * 2).collect::<String>()
}