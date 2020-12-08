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

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    
    let url = "https://products.dm.de/store-availability/DE/availability?dans=28171,45784&storeNumbers=2787";
    
    let res: Availabilities = reqwest::get(url).await?.json().await?;

    println!("{:?}", res);
    println!("{:?}", res.storeAvailabilities.get(&28171).unwrap()[0].stockLevel);
    Ok(())
}
