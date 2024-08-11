use crate::parser::error::Error;
use crate::parser::ws;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, tag_no_case};
use nom::character::complete::{char, u64};
use nom::combinator::map;
use nom::error::context;
use nom::multi::{many0, separated_list1};
use nom::sequence::delimited;
use nom::{Finish, IResult};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Name(String),
    Id(u64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Filter {
    Guild(Vec<Value>),
    From(Vec<Value>),
}

impl Filter {
    fn is_guild(filter: &Filter) -> Option<Vec<Value>> {
        match filter {
            Filter::Guild(v) => Some(v.to_owned()),
            Filter::From(_) => None,
        }
    }

    fn is_from(filter: &Filter) -> Option<Vec<Value>> {
        match filter {
            Filter::Guild(_) => None,
            Filter::From(v) => Some(v.to_owned()),
        }
    }
}

fn parse_single_quotes(input: &str) -> IResult<&str, Value> {
    map(delimited(char('\''), is_not("'"), char('\'')), |v: &str| Value::Name(v.to_string()))(input)
}

fn parse_double_quotes(input: &str) -> IResult<&str, Value> {
    map(delimited(char('"'), is_not("\""), char('"')), |v: &str| Value::Name(v.to_string()))(input)
}

fn parse_string(input: &str) -> IResult<&str, Value> {
    alt((parse_double_quotes, parse_single_quotes))(input)
}

// TODO: make it so it will error if passed "12abc23"
fn parse_number(input: &str) -> IResult<&str, Value> {
    map(u64, |v| Value::Id(v))(input)
}

fn parse_values(input: &str) -> IResult<&str, Vec<Value>> {
    separated_list1(ws(tag("||")), alt((parse_string, parse_number)))(input)
}

fn parse_guild(input: &str) -> IResult<&str, Filter> {
    context(
        "guild",
        map(delimited(tag_no_case("guild("), ws(parse_values), tag(")")), |v| Filter::Guild(v)),
    )(input)
}

fn parse_from(input: &str) -> IResult<&str, Filter> {
    context(
        "from",
        map(delimited(tag_no_case("from("), ws(parse_values), tag(")")), |v| Filter::From(v)),
    )(input)
}

fn either(input: &str) -> IResult<&str, Filter> {
    alt((parse_guild, parse_from))(input)
}

fn parse_query(input: &str) -> IResult<&str, Vec<Filter>> {
    context("query", many0(ws(either)))(input)
}

#[derive(Debug)]
pub struct Query {
    pub guild: Vec<Value>,
    pub from: Vec<Value>,
}

impl Query {
    pub fn from_str(input: &str) -> Result<Self, Error> {
        // TODO: for some reason errors are not propagated.
        // E.g `from_str("guild(") - should return error
        let (_, result) = parse_query(input).finish().map_err(|e| Error::Failed(e.code))?;

        let guild: Vec<_> = result.iter().filter_map(Filter::is_guild).flatten().collect();
        let from: Vec<_> = result.iter().filter_map(Filter::is_from).flatten().collect();

        Ok(Self { guild, from })
    }
}

#[cfg(test)]
mod parser_test {
    use crate::parser::parser::{parse_double_quotes, parse_number, parse_single_quotes, parse_string, Value};
    use rstest::rstest;

    #[rstest]
    #[case("'abc'", Value::Name(String::from("abc")))]
    #[case("'123'", Value::Name(String::from("123")))]
    #[case("'va\"la'", Value::Name(String::from("va\"la")))]
    fn single_quotes(#[case] input: &str, #[case] expected: Value) {
        assert_eq!(parse_single_quotes(input), Ok(("", expected)))
    }

    #[rstest]
    #[case("\"abc\"", Value::Name(String::from("abc")))]
    #[case("\"123\"", Value::Name(String::from("123")))]
    #[case("\"va'la\"", Value::Name(String::from("va'la")))]
    fn double_quotes(#[case] input: &str, #[case] expected: Value) {
        assert_eq!(parse_double_quotes(input), Ok(("", expected)))
    }

    #[rstest]
    #[case("\"hello world\"", Value::Name(String::from("hello world")))]
    #[case("'hello world'", Value::Name(String::from("hello world")))]
    fn string(#[case] input: &str, #[case] expected: Value) {
        assert_eq!(parse_string(input), Ok(("", expected)))
    }

    #[rstest]
    #[case("123", Value::Id(123))]
    fn number(#[case] input: &str, #[case] expected: Value) {
        assert_eq!(parse_number(input), Ok(("", expected)))
    }
}
