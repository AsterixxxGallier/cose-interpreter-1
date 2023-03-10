use std::fmt::{Debug, Formatter};
use linked_spaced_list::{Bound, LinkedRangeSpacedList};
use pest::iterators::{Pair, Pairs};
use pest::{Parser, Span};
use pest_derive::Parser;
use crate::parser::Expression::{Association, Cose, PrefixReference, Reference, Text};
use crate::Set;

#[derive(Parser)]
#[grammar = "cose.pest"]
pub struct CoseParser;

pub mod error;

#[derive(Debug)]
pub enum Expression {
    Association {
        parent: Option<usize>,
        keys: Set,
        values: Set,
    },
    Reference {
        parent: Option<usize>,
        associations: Set,
        keys: Set,
    },
    PrefixReference {
        parent: Option<usize>,
        keys: Set,
    },
    Cose,
    Text(String),
}

pub(crate) struct Builder<'s> {
    pub(crate) expressions: LinkedRangeSpacedList<Expression>,
    pub(crate) top_level: Set,
    pub(crate) source: &'s str
}

impl<'s> Builder<'s> {
    pub fn new(source: &'s str) -> Result<Self, pest::error::Error<Rule>> {
        let mut this = Self {
            expressions: LinkedRangeSpacedList::new(),
            top_level: Set::new(),
            source
        };
        let file = CoseParser::parse(Rule::file, source)?.next().unwrap();
        let mut top_level = this.expressions(file.into_inner(), None);
        this.top_level.append(&mut top_level);
        Ok(this)
    }

    fn push(&mut self, start: usize, end: usize, expression: Expression) -> usize {
        self.expressions.insert_surrounding(start, end, expression).0
    }

    fn push_span(&mut self, span: Span, expression: Expression) -> usize {
        self.push(span.start(), span.end(), expression)
    }

    fn expressions(&mut self, pairs: Pairs<Rule>, parent: Option<usize>) -> Set {
        pairs
            .flat_map(|pair: Pair<Rule>|
                if pair.as_rule() == Rule::enclosed_expressions {
                    pair.into_inner().collect()
                } else {
                    vec![pair]
                }
            )
            .map(|pair| self.expression(pair, parent)).collect()
    }

    fn expression(&mut self, pair: Pair<Rule>, parent: Option<usize>) -> usize {
        match pair.as_rule() {
            Rule::association => self.association(pair, parent),
            Rule::reference => self.reference(pair, parent),
            Rule::cose => self.cose(pair),
            Rule::text => self.text(pair),
            _ => panic!()
        }
    }

    fn reference(&mut self, pair: Pair<Rule>, parent: Option<usize>) -> usize {
        let pair = pair.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::prefix_reference => self.prefix_reference(pair, parent),
            Rule::multi_reference => self.multi_reference(pair, parent),
            _ => unreachable!()
        }
    }

    fn prefix_reference(&mut self, pair: Pair<Rule>, parent: Option<usize>) -> usize {
        let span = pair.as_span();
        let keys = self.referenceables(pair.into_inner().next().unwrap().into_inner(), parent);
        self.push_span(span, PrefixReference { parent, keys })
    }

    fn multi_reference(&mut self, pair: Pair<Rule>, parent: Option<usize>) -> usize {
        let start = pair.as_span().start();
        let mut elements = pair.into_inner();
        let first_element = elements.next().unwrap();
        let mut associations = self.referenceables(first_element.into_inner(), parent);
        while let Some(keys) = elements.next() {
            let end = keys.as_span().end();
            let keys = self.referenceables(keys.into_inner(), parent);
            associations = vec![self.push(start, end, Reference { parent, associations, keys })];
        }
        associations[0]
    }

    fn referenceables(&mut self, mut pairs: Pairs<Rule>, parent: Option<usize>) -> Vec<usize> {
        match pairs.peek().unwrap().as_rule() {
            Rule::prefix_reference => vec![self.prefix_reference(pairs.next().unwrap(), parent)],
            _ => self.expressions(pairs, parent)
        }
    }

    fn association(&mut self, pair: Pair<Rule>, parent: Option<usize>) -> usize {
        let index = self.push_span(pair.as_span(), Association { parent, keys: Set::new(), values: Set::new() });
        let mut inner = pair.into_inner();
        let keys = self.expressions(inner.next().unwrap().into_inner(), parent);
        let values = self.expressions(inner.next().unwrap().into_inner(), Some(index));
        if let Association {
            keys: keys_,
            values: values_,
            ..
        } = &mut self.expressions[index] {
            *keys_ = keys;
            *values_ = values;
        } else { unreachable!() }
        index
    }

    fn cose(&mut self, pair: Pair<Rule>) -> usize {
        self.push_span(pair.as_span(), Cose)
    }

    fn text(&mut self, pair: Pair<Rule>) -> usize {
        self.push_span(pair.as_span(), Text(pair.into_inner().map(Self::text_component).collect()))
    }

    fn text_component(pair: Pair<Rule>) -> char {
        match pair.as_rule() {
            Rule::escaped_char => pair.as_str().chars().next().unwrap(),
            Rule::valid_text_char => pair.as_str().chars().next().unwrap(),
            _ => unreachable!()
        }
    }
}

impl<'s> Debug for Builder<'s> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (index, position, expression) in self.expressions.indexed() {
            match expression {
                Bound::Start { value, .. } => {
                    writeln!(f, "{:2} starts at {}: {:?}", index, position, value)?;
                }
                Bound::End { start } => {
                    writeln!(f, "{:2} ends at {}", start, position)?;
                }
            }
        }
        Ok(())
    }
}
