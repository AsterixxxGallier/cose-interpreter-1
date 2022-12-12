use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "cose.pest"]
pub struct CoseParser;

pub mod error;


