mod lex;
mod parse;

use parse::AstEntityNode;
use parse::AstFieldNode;
use select::lex::lex_selector;
use select::parse::{SelectorParser, Selector};
use parse::ParseResult;


pub fn select_entity<'a>(pr: &'a ParseResult, selector_string: &str) -> Vec<&'a AstEntityNode> {
    let tokens = lex_selector(selector_string);
    let parser = SelectorParser::new(tokens);
    let selector = parser.parse().unwrap();

    let mut result = vec![];

    for ent in &pr.entities {
        if matches_selector(&ent, &selector) {
            result.push(ent);
        }
    }

    let mut current_selector = selector;
    while current_selector.child.is_some() {
        current_selector = *current_selector.child.unwrap();
        let sub_results = select_in_entities(result, &current_selector, pr);
        result = sub_results;
    }

    return result;
}

pub fn select_field<'a>(root: &'a ParseResult, selector_string: &str) -> Vec<&'a AstFieldNode> {
    let tokens = lex_selector(selector_string);
    let parser = SelectorParser::new(tokens);
    let mut selector = parser.parse().unwrap();

    let mut current_set = Vec::new();
    for e in &root.entities {
        current_set.push(e);
    }

    // first pass , check in root entities
    if selector.child.is_some() {
        let mut next_set = Vec::new();
        for e in current_set {
            if matches_selector(e, &selector) {
                next_set.push(e);
            }
        }
        current_set = next_set;
        selector = *selector.child.unwrap();
    }

    // pass 2 -> n , check in the current set
    while selector.child.is_some() {
        let next_set = select_in_entities(current_set, &selector, root);
        current_set = next_set;
        selector = *selector.child.unwrap();
    }


    // got to the last selector , should be a field selector
    let mut result = Vec::new();
    let mut fields = Vec::new();
    for entity in current_set {
        for field_ref in &entity.fields {
            fields.push(root.get_field(*field_ref));
        }
    }
    for field in fields {
        match selector.identifier {
            Some(ref id) => {
                if id == &field.identifier {
                    result.push(field);
                }
            }
            None => {}
        }
    }

    return result;
}

fn select_in_entities<'a>(entities: Vec<&AstEntityNode>, selector: &Selector, pr: &'a ParseResult) -> Vec<&'a AstEntityNode> {
    let mut result = vec![];
    for entity in entities {
        for child_id in &entity.children {
            let child = pr.get_entity(*child_id);
            if matches_selector(child, selector) {
                result.push(child);
            }
            let mut sub_results = select_in_entities(vec![child], selector, pr);
            if sub_results.len() > 0 {
                result.append(&mut sub_results);
            }
        }
    }
    return result;
}


fn matches_selector(header: &AstEntityNode, selector: &Selector) -> bool {
    let matches = true;
    match selector.main_type {
        Some(ref s) => {
            if &header.main_type != s {
                return false;
            }
        }
        None => {}
    }
    match selector.sub_type {
        Some(ref s) => {
            match header.sub_type {
                Some(ref hs) => {
                    if hs != s {
                        return false;
                    }
                }
                None => {
                    // matching on an sub_type, but entity has none, no match
                    return false;
                }
            }
        }
        None => {}
    }
    match selector.identifier {
        Some(ref s) => {
            match header.identifier {
                Some(ref hi) => {
                    if hi != s {
                        return false;
                    }
                }
                None => {
                    // matching on an identifier, but entity has none, no match
                    return false;
                }
            }
        }
        None => {}
    }

    return matches;
}


#[cfg(test)]
mod test {
    use lex::Lexer;
    use parse::Parser;
    use select::select_entity;
    use select::select_field;

    #[test]
    fn select_entity_simple() {
        let cdl = "
widget kpi {
    label : \"Label\"
    labels : \"Labels\"
}

widget kpi2 {
    label : \"Label\"
    labels : \"Labels\"
}
".to_string();
        let lexer = Lexer::new(cdl);
        let lex_items = lexer.lex().unwrap();
        let parser = Parser::new(lex_items);
        let root = parser.parse().unwrap();

        let result = select_entity(&root, "widget[kpi]");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn select_entity_simple2() {
        let cdl = "
page {

    widget kpi {
        label : \"Label\"
        labels : \"Labels\"
    }
}

page {

    widget kpi {
        label : \"Label\"
        labels : \"Labels\"
    }
    widget kpi2 {
        label : \"Label\"
        labels : \"Labels\"
    }
    widget kpi3 #kpiid {
        label : \"Label\"
        labels : \"Labels\"
    }
}
".to_string();
        let lexer = Lexer::new(cdl);
        let lex_items = lexer.lex().unwrap();
        let parser = Parser::new(lex_items);
        let pr = parser.parse().unwrap();

        assert_eq!(select_entity(&pr, "widget[kpi]").len(), 2);
        assert_eq!(select_entity(&pr, "widget[kpi2]").len(), 1);
        assert_eq!(select_entity(&pr, "widget").len(), 4);
        assert_eq!(select_entity(&pr, "widget.kpiid").len(), 1);
    }


    #[test]
    fn select_nested_entity() {
        let cdl = "
page {

    widget kpi {
        label : \"Label\"
        labels : \"Labels\"
    }
}

page {

    widget kpi {
        label : \"Label\"
        labels : \"Labels\"
    }
    widget kpi2 {
        label : \"Label\"
        labels : \"Labels\"
    }
    widget kpi3 #kpiid {
        label : \"Label\"
        labels : \"Labels\"
    }
}
".to_string();
        let lexer = Lexer::new(cdl);
        let lex_items = lexer.lex().unwrap();
        let parser = Parser::new(lex_items);
        let root = parser.parse().unwrap();

        assert_eq!(select_entity(&root, "page > widget").len(), 4);
        assert_eq!(select_entity(&root, "page > widget[kpi]").len(), 2);
        assert_eq!(select_entity(&root, "page > widget[kpi2]").len(), 1);
    }

    #[test]
    fn select_field_simple() {
        let cdl = "
    page {

        widget kpi {
            label : \"Label\"
            labels : \"Labels\"
        }
    }

    page {

        widget kpi {
            label : \"Label\"
            labels : \"Labels\"
        }
        widget kpi2 {
            label : \"Label\"
            labels : \"Labels\"
        }
        widget kpi3 #kpiid {
            label : \"Label\"
            labels : \"Labels\"
        }
    }
    ".to_string();
        let lexer = Lexer::new(cdl);
        let lex_items = lexer.lex().unwrap();
        let parser = Parser::new(lex_items);
        let root = parser.parse().unwrap();

        assert_eq!(select_field(&root, ".label").len(), 4);
        assert_eq!(select_field(&root, "page > widget[kpi3] > .label").len(), 1);
        assert_eq!(select_field(&root, "widget > .label").len(), 4);
    }
}


