#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use crate::models::NewStock;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use chrono::Local;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn save_stock<'a>(conn: &MysqlConnection, product_type: &'a String, amount: &'a i32, store_number: &'a i32) -> usize{
    use schema::stock;

    let time = Local::now().naive_local();
    let new_stock = NewStock { product_type, time: &time, amount, store_number};

    diesel::insert_into(stock::table).values(&new_stock).execute(conn).expect("Error saving stock")
}
