#![allow(unused)]

use linked_spaced_list::LinkedRangeSpacedList;
use crate::parser::{Builder, Expression, Rule};

pub(crate) mod parser;

type Set = Vec<usize>;

pub struct Interpreter<'s> {
    expressions: LinkedRangeSpacedList<Expression>,
    top_level: Set,
    source: &'s str,
}

impl<'s> Interpreter<'s> {
    pub fn new(source: &'s str) -> Result<Self, pest::error::Error<Rule>> {
        Builder::new(source).map(Self::from_builder)
    }

    fn from_builder(builder: Builder<'s>) -> Self {
        Self {
            expressions: builder.expressions,
            top_level: builder.top_level,
            source: builder.source
        }
    }
}

fn main() {
    let builder = Builder::new(r#"
(a): >b>c
   b: c
    "#).unwrap();
    println!("{:#?}", builder);
}
