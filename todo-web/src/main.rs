#[macro_use]
extern crate rocket;

use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TodoItem {
    id: u32,
    text: String,
}

async fn get_items() -> Vec<TodoItem> {
    let url = Url::parse("http://todo-api:8000").unwrap();
    let items = reqwest::get(url)
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

    let mut html = String::from("<html><body><ul>");
    for item in items {
        html.push_str(&format!("<li>{}</li>", item.text));
    }
    html.push_str("</ul></body></html>");

    html
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])

    // LOCAL TEST
    // let figment = rocket::Config::figment()
    //     .merge(("port", 8080))
    //     .merge(("address", "0.0.0.0"));
    // rocket::custom(figment).mount("/", routes![index])
}
