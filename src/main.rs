use pest::Parser;

pub(crate) mod parser;

fn main() {
    let ast = parser::CoseParser::parse(parser::Rule::file, r#"
a: b>c
    "#);
    println!("{:#?}", ast);
}
