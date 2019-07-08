-- Your SQL goes here


CREATE TABLE pharmacies_sales_products(
    quantity INTEGER NOT NULL,
    date VARCHAR NOT NULL,
    price INTEGER NOT NULL,
    pharmacies_id SERIAL references pharmacies(id),
    products_id SERIAL references products(id),
    id SERIAL PRIMARY KEY
);