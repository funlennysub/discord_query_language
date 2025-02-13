use nom::character::complete::multispace0;
use nom::error::ParseError;
use nom::sequence::delimited;
use nom::IResult;

pub mod error;
pub mod parser;

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
