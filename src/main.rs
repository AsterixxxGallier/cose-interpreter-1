use crate::parser::Builder;

pub(crate) mod parser;

fn main() {
    let mut builder = Builder::new();
    builder.file(r#"
a: >b>c
   b: c
    "#).unwrap();
    println!("{:#?}", builder);
}
