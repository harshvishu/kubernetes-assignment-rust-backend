#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct TodoItem {
    id: u32,
    body: String,
    completed: bool,
}

async fn get_items() -> Vec<TodoItem> {
    let todo_api_host = env::var("todo_api_host").expect("todo_api_host URL must be set");
    let todo_api_port = env::var("todo_api_port").expect("todo_api_port URL must be set");

    let url = format!("http://{}:{}", todo_api_host, todo_api_port);
    let url = Url::parse(&url).unwrap();
    let items = reqwest::get(url)
        .await
        .unwrap()
        .json::<Vec<TodoItem>>()
        .await
        .unwrap();

    items
}

async fn post_items(todo: String) -> Vec<TodoItem> {
    let url = Url::parse("http://todo-api:8000/todo").unwrap();
    let client = reqwest::Client::new();

    let items = client
        .post(url)
        .body(todo)
        .send()
        .await
        .unwrap()
        .json::<Vec<TodoItem>>()
        .await
        .unwrap();

    items
}

#[get("/")]
async fn index() -> String {
    let items = get_items().await;

    let mut html = String::new();
    html.push_str("<html>");
    html.push_str("<head><title>Todo Items</title></head>");
    html.push_str("<body>");
    html.push_str("<h1>Todo Items:</h1>");
    html.push_str("<ul>");

    for item in items {
        html.push_str(&format!(
            "<li>Title: {}. Completed {}</li>",
            item.body, item.completed
        ));
    }

    html.push_str("</ul>");
    html.push_str("</body>");
    html.push_str("</html>");

    html
}

#[post("/todo", data = "<todo>")]
async fn create_todo(todo: &str) -> String {
    let items = post_items(todo.to_owned()).await;

    let mut html = String::new();
    html.push_str("<html>");
    html.push_str("<head><title>Todo Items</title></head>");
    html.push_str("<body>");
    html.push_str("<h1>New Todo Created</h1>");
    html.push_str("<ul>");

    for item in items {
        html.push_str(&format!(
            "<li>Title: {}. Completed {}</li>",
            item.body, item.completed
        ));
    }

    html.push_str("</ul>");
    html.push_str("</body>");
    html.push_str("</html>");

    html
}

#[launch]
fn rocket() -> _ {
    // rocket::build().mount("/", routes![index])

    dotenv().ok();

    // LOCAL TEST
    let figment = rocket::Config::figment()
        .merge(("port", 8000))
        .merge(("address", "0.0.0.0"));
    rocket::custom(figment).mount("/", routes![index, create_todo])
}
