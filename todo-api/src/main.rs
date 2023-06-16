#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;
use self::models::*;

#[get("/")]
fn get_items() -> String {
    let conn = &mut establish_connection();
    let items = show_todos(conn);
    serde_json::to_string(&items).unwrap()
}

#[post("/todo", data = "<todo>")]
fn create_items(todo: &str) -> String {
    let conn = &mut establish_connection();
    let todo: NewTodoItem = todo.into();
    let created_todos = create_todos(conn, vec![todo]);
    serde_json::to_string(&created_todos).unwrap()
}

#[launch]
fn rocket() -> _ {
    // rocket::build().mount("/", routes![get_items])
    // LOCAL TEST
    let figment = rocket::Config::figment()
        .merge(("port", 8000))
        .merge(("address", "0.0.0.0"));
    rocket::custom(figment).mount("/", routes![get_items, create_items])
}

// DIESEL STUFF

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// TODOS 

pub fn create_todos(conn: &mut PgConnection, entries: Vec<NewTodoItem>) -> Vec<TodoItem> {
    use crate::schema::todos;

    diesel::insert_into(todos::table)
        .values(&entries)
        .returning(TodoItem::as_returning())
        .get_results(conn)
        .expect("Error saving todos")
}

pub fn show_todos(conn: &mut PgConnection) -> Vec<TodoItem> {
    use self::schema::todos::dsl::*;

    let results = todos
        .select(TodoItem::as_select())
        .load(conn)
        .expect("Error loading todos");

    return results;
}