mod lex;
mod parse;

use parse::AstEntityNode;
use parse::AstRootNode;
use parse::AstEntityHeaderNode;
use parse::AstFieldNode;
use select::lex::lex_selector;
use select::parse::{SelectorParser, Selector};


pub fn select_entity<'a>(root: &'a AstRootNode, selector_string: &str) -> Vec<&'a AstEntityNode> {
    let tokens = lex_selector(selector_string);
    let parser = SelectorParser::new(tokens);
    let selector = parser.parse().unwrap();

    let mut result = vec![];

    for child in &root.children {
        let mut sub_results = select_in_entities(vec![child], &selector);
        if sub_results.len() > 0 {
            result.append(&mut sub_results);
        }
    }
    let mut current_selector = selector;
    while current_selector.child.is_some() {
        current_selector = *current_selector.child.unwrap();
        let sub_results = select_in_entities(result, &current_selector);
        result = sub_results;
    }

    return result;
}

pub fn select_field<'a>(root: &'a AstRootNode, selector_string: &str) -> Vec<&'a AstFieldNode> {
    let tokens = lex_selector(selector_string);
    let parser = SelectorParser::new(tokens);
    let mut selector = parser.parse().unwrap();

    let mut intermediate = Vec::new();

    for e in &root.children {
        intermediate.push(e);
    }

    while selector.child.is_some() {
        let sub_results = select_in_entities(intermediate, &selector);
        intermediate = sub_results;
        selector = *selector.child.unwrap();
    }


    let result = select_field_in_entity(intermediate, &selector);
    return result;
}

fn select_in_entities<'a>(entities: Vec<&'a AstEntityNode>, selector: &Selector) -> Vec<&'a AstEntityNode> {
    let mut result = vec![];

    for entity in entities {
        if matches_selector(&entity.header, &selector) {
            result.push(entity);
        }

        for child in &entity.body.children {
            let mut sub_results = select_in_entities(vec![child], selector);
            if sub_results.len() > 0 {
                result.append(&mut sub_results);
            }
        }
    }

    return result;
}

fn select_field_in_entity<'a>(entities: Vec<&'a AstEntityNode>, selector: &Selector) -> Vec<&'a AstFieldNode> {
    let mut result = vec![];
    for entity in entities {
        for field in &entity.body.fields {
            match selector.identifier {
                Some(ref id) => {
                    if id == &field.identifier {
                        result.push(field);
                    }
                }
                None => {}
            }
        }

        for child in &entity.body.children {
            let mut sub_results = select_field_in_entity(vec![child], &selector);
            if sub_results.len() > 0 {
                result.append(&mut sub_results);
            }
        }
    }
    return result;
}

fn matches_selector(header: &AstEntityHeaderNode, selector: &Selector) -> bool {
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
    use select::{select_entity, select_field};

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
        let root = parser.parse().unwrap();

        assert_eq!(select_entity(&root, "widget[kpi]").len(), 2);
        assert_eq!(select_entity(&root, "widget[kpi2]").len(), 1);
        assert_eq!(select_entity(&root, "widget").len(), 4);
        assert_eq!(select_entity(&root, "widget.kpiid").len(), 1);
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


