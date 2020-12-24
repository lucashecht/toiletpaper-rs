# Toilet Paper Tracker

## Usage ##

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
