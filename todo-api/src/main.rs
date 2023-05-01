#[macro_use]
extern crate rocket;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TodoItem {
    id: u32,
    text: String,
}

#[get("/")]
fn get_items() -> String {
    let items = vec![
        TodoItem {
            id: 1,
            text: String::from("Buy milk"),
        },
        TodoItem {
            id: 2,
            text: String::from("Do laundry"),
        },
        TodoItem {
            id: 3,
            text: String::from("Clean room"),
        },
    ];

    serde_json::to_string(&items).unwrap()
}

#[launch]
fn rocket() -> _ {
    // rocket::build().mount("/", routes![get_items])

    // LOCAL TEST
    let figment = rocket::Config::figment()
        .merge(("port", 8000))
        .merge(("address", "0.0.0.0"));
    rocket::custom(figment).mount("/", routes![get_items])
}
