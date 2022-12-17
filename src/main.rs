use crate::parser::Builder;

pub(crate) mod parser;

fn main() {
    let builder = Builder::new(r#"
(a): >b>c
   b: c
    "#).unwrap();
    println!("{:#?}", builder);
}
