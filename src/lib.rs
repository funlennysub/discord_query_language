use crate::parser::parser::Query;

pub mod parser;
mod backend;

pub trait FromQuery {
    type Source;
    type Output;
    type Error: std::error::Error;

    fn select(query: Query, source: Self::Source) -> Result<Self::Output, Self::Error>;
}