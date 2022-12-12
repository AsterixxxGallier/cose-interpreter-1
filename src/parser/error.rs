use crate::parser::Rule;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    ParseError(pest::error::Error<Rule>),
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(error: pest::error::Error<Rule>) -> Self {
        Self::ParseError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error)
    }
}
