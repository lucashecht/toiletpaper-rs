CREATE TABLE stock (
  id SERIAL PRIMARY KEY,
  product_type VARCHAR(30) NOT NULL,
  time DATETIME NOT NULL,
  amount INTEGER NOT NULL
)
