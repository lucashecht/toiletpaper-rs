#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use crate::models::NewStock;

use chrono::Local;
use diesel::prelude::*;
use dotenv::dotenv;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct ItemAttributes {
    inStock: bool,
    stockLevel: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Availabilities {
    storeAvailabilities: HashMap<i32, Vec<ItemAttributes>>,
}

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn build_url(store_number: i32, item_ids: Vec<&str>) -> String {
    let url = format!("https://products.dm.de/store-availability/DE/availability?dans={items}&storeNumbers={store}",
                      items = item_ids.join(","),
                      store = store_number.to_string());

    url
}

fn sum_stocklevels(items: HashMap<i32, Vec<ItemAttributes>>) -> i32 {
    let mut sum: i32 = 0;

    for (_item, attributes) in &items {
        sum += attributes[0].stockLevel;
    }

    sum
}

async fn get_stock(store_number: i32, item_ids: Vec<&str>) -> i32 {
    let url = build_url(store_number, item_ids);
    let response = match reqwest::get(&url).await {
        Ok(resp) => resp,
        Err(_) => {
            println!("Error fetching stock");
            return 0;
        }
    };

    match response.json::<Availabilities>().await {
        Ok(result) => sum_stocklevels(result.storeAvailabilities),
        Err(_) => {
            println!("Error fetching stock");
            0
        }
    }
}

pub async fn get_toiletpaper_stock() -> i32 {
    let store_number = 2787;
    let items = vec![
        "485698", "709006", "452744", "137425", "28171", "595420", "799358", "863567", "610544",
        "452740", "846857", "452753", "525943", "879536", "485695", "853483", "594080", "504606",
        "593761", "535981", "842480", "524535", "127048", "524533", "524532", "846834", "708997",
        "711915",
    ];

    get_stock(store_number, items).await
}

pub fn save_stock<'a>(product_type: &'a String, amount: &'a i32, store_number: &'a i32) -> usize {
    use schema::stock;

    let conn = establish_connection();

    let time = Local::now().naive_local();
    let new_stock = NewStock {
        product_type,
        time: &time,
        amount,
        store_number,
    };

    diesel::insert_into(stock::table)
        .values(&new_stock)
        .execute(&conn)
        .expect("Error saving stock")
}
