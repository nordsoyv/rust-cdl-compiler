mod lex;
mod parse;
mod print;
mod select;

use parse::AstRootNode;
use parse::Parser;
pub use lex::Lexer;
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