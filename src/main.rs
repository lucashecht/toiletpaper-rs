#![feature(proc_macro_hygiene, decl_macro)]

use toiletpaper::{establish_connection, save_stock};
use serde::Deserialize;
use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct ItemAttributes {
    inStock: bool,
    stockLevel: i32,
}

#[macro_use] extern crate rocket;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Availabilities {
    storeAvailabilities: HashMap<i32, Vec<ItemAttributes>>,
}

#[get("/")]
async fn stock() -> String {
    get_toiletpaper_stock().await.to_string()
}

fn sum_stocklevels(items: HashMap<i32, Vec<ItemAttributes>>) -> i32 {
    let mut sum: i32 = 0;

    for (_item, attributes) in &items {
        sum += attributes[0].stockLevel;
    }

    sum
}

fn build_url(store_number: i32, item_ids: Vec<&str>) -> String { 
    let url = format!("https://products.dm.de/store-availability/DE/availability?dans={items}&storeNumbers={store}",
                      items = item_ids.join(","),
                      store = store_number.to_string());
    
    url
}

async fn get_stock(store_number: i32, item_ids: Vec<&str>) -> i32 {
    let url = build_url(store_number, item_ids);
    let response = match reqwest::get(&url).await {
        Ok(resp) => resp,
        Err(_) => {
            println!("Error fetching stock");
            return 0
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

async fn get_toiletpaper_stock() -> i32 {
    let store_number = 2787;
    let items = vec!["485698", "709006", "452744", "137425", "28171", "595420", "799358", "863567", "610544", "452740", "846857", "452753", "525943", "879536", "485695", "853483", "594080", "504606", "593761", "535981", "842480", "524535", "127048", "524533", "524532", "846834", "708997", "711915"];

    get_stock(store_number, items).await
}

#[rocket::main]
async fn main() {
    
    let stocklevel = get_toiletpaper_stock().await;
    println!("Stock level: {}", stocklevel);

    /*let connection = establish_connection();
    let product_type = String::from("toiletpaper");
    save_stock(&connection, &product_type, &stocklevel, &store_number);*/

    rocket::ignite().mount("/", routes![stock]).launch().await;
}
