use crate::schema::stock;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Stock {
    pub id: i32,
    pub product_type: String,
    pub time: NaiveDateTime,
    pub amount: i32,
    pub store_number: i32,
}

#[derive(Insertable)]
#[table_name="stock"]
pub struct NewStock<'a> {
    pub product_type: &'a String,
    pub time: &'a NaiveDateTime,
    pub amount: &'a i32,
    pub store_number: &'a i32,
}
  
