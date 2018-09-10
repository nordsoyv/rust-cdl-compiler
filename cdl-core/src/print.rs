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
    res.push_str(&print_expr(&field.value));
    res.push_str("\n");
    res
}

fn print_expr(expr: &Expr) -> String {
    let mut res = "".to_string();

    match expr {
        Expr::Operator(node) => {
            let left_side = print_expr(&node.left_side);
            let right_side = print_expr(&node.right_side);
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
            res.push_str(&print_expr(&node.expr));
        }
        Expr::Number(node) => {
            res.push_str(&node.text_rep);
        }
        Expr::Function(node) => {
            let mut arg_list = Vec::new();
            res.push_str(&node.identifier);
            res.push('(');
            for arg in &node.argument_list {
                arg_list.push(print_expr(&arg));
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