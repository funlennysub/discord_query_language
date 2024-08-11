use crate::parser::parser::{Query, Value};
use crate::FromQuery;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use thiserror::Error;

pub struct FromJson;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Guild {
    id: u64,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u64,
    username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Row {
    guild: Guild,
    message: String,
    author: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data(Vec<Row>);

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

fn is_same_user(user: &User, value: &Value) -> bool {
    match value {
        Value::Name(n) => &user.username == n,
        Value::Id(id) => &user.id == id,
    }
}

fn is_same_guild(guild: &Guild, value: &Value) -> bool {
    match value {
        Value::Name(n) => &guild.name == n,
        Value::Id(id) => &guild.id == id,
    }
}

impl FromQuery for FromJson {
    type Source = Vec<u8>;
    type Error = Error;
    type Output = Data;

    fn select(query: Query, source: Self::Source) -> Result<Self::Output, Self::Error> {
        let json: Data = serde_json::from_slice(&source)?;
        if query.from.is_empty() && query.guild.is_empty() {
            return Ok(json);
        }

        let mut new_json = vec![];

        'row: for row in &json.0 {
            for from in &query.from {
                if is_same_user(&row.author, from) {
                    new_json.push(row.to_owned());
                    continue 'row;
                }
            }

            for guild in &query.guild {
                if is_same_guild(&row.guild, guild) {
                    new_json.push(row.to_owned());
                    continue 'row;
                }
            }
        }

        Ok(Data(new_json))
    }
}

#[cfg(test)]
mod json_test {
    use crate::backend::json::{Data, FromJson, Guild, Row, User};
    use crate::parser::parser::Query;
    use crate::FromQuery;

    #[test]
    fn parse_json() {
        let row1 = Row {
            guild: Guild {
                id: 1234,
                name: "test".to_string(),
            },
            message: "123".to_string(),
            author: User {
                id: 5678,
                username: "funlennysub".to_string(),
            },
        };
        let row2 = Row {
            guild: Guild {
                id: 64773,
                name: "test1".to_string(),
            },
            message: "1235".to_string(),
            author: User {
                id: 5678,
                username: "funlennysub".to_string(),
            },
        };

        let json = Data(vec![row1, row2]);
        let str = serde_json::to_vec(&json).unwrap();
        let from = FromJson::select(Query::from_str("guild(1234) from(5678)").unwrap(), str).unwrap();

        assert_eq!(2, from.0.len())
    }
}
