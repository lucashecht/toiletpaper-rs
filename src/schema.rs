table! {
    stock (id) {
        id -> Unsigned<Bigint>,
        product_type -> Varchar,
        time -> Datetime,
        amount -> Integer,
    }
}
