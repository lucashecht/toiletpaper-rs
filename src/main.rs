use toiletpaper::{establish_connection, save_stock};
use serde::Deserialize;
use std::collections::HashMap;

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
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let store_number = 2787;
    let items = vec!["485698", "709006", "452744", "137425", "28171", "595420", "799358", "863567", "610544", "452740", "846857", "452753", "525943", "879536", "485695", "853483", "594080", "504606", "593761", "535981", "842480", "524535", "127048", "524533", "524532", "846834", "708997", "711915"];

    let url = build_url(store_number, items);
    let res: Availabilities = reqwest::get(&url).await?.json().await?;

    let stocklevel = sum_stocklevels(res.storeAvailabilities);

    println!("Stock level: {}", stocklevel);

    let connection = establish_connection();
    let product_type = String::from("toiletpaper");
    save_stock(&connection, &product_type, &stocklevel, &store_number);
    Ok(())
}
