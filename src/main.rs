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


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://products.dm.de/store-availability/DE/availability?dans=28171,45784&storeNumbers=2787";
    
    let res: Availabilities = reqwest::get(url).await?.json().await?;

    let stocklevel = sum_stocklevels(res.storeAvailabilities);

    println!("Stock level: {}", stocklevel);
    Ok(())
}
