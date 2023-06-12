use std::str::FromStr;

use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
#[derive(Serialize)]
pub struct TodoItem {
    pub id: i32,
    pub body: String,
    pub completed: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::todos)]
#[derive(Debug)]
pub struct NewTodoItem {
    pub body: String,
}

impl FromStr for NewTodoItem {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(NewTodoItem {
            body: s.parse::<String>()?,
        });
    }
}

impl From<&str> for NewTodoItem {
    fn from(body: &str) -> Self {
        NewTodoItem {
            body: body.to_string(),
        }
    }
}