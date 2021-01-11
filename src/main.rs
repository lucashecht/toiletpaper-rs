#![feature(proc_macro_hygiene, decl_macro)]

use toiletpaper::{save_stock, get_toiletpaper_stock};

#[macro_use]
extern crate rocket;

#[get("/")]
async fn stock() -> String {
    get_toiletpaper_stock().await.to_string()
}

#[rocket::main]
async fn main() {
    let stocklevel = get_toiletpaper_stock().await;
    println!("Stock level: {}", stocklevel);

    /*let product_type = String::from("toiletpaper");
    save_stock(&product_type, &stocklevel, &store_number);*/

    rocket::ignite().mount("/", routes![stock]).launch().await;
}
