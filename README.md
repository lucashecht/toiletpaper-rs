# Toilet Paper Tracker

## About

This is my first Rust project with the main objective to learn the language, I don't actually need to constantly know how much toiletpaper there is.
I will add to the project as I learn new things.

## Usage

Set the url of your database:

```sh
echo DATABASE_URL=mysql://[USERNAME]:[PASSWORD]@localhost/toiletpaper > .env
```

Initialize the database:

```sh
diesel migration run
```

Run the program:

```sh
cargo run
```

## ToDo

* Add support for other products
* Create webpage
* Add tests
