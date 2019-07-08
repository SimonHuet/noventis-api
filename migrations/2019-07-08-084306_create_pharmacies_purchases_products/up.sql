-- Your SQL goes here


CREATE TABLE pharmacies_purchases_products(
    quantity INTEGER NOT NULL,
    date VARCHAR NOT NULL,
    price INTEGER NOT NULL,
    pharmacies_id SERIAL references pharmacies(id),
    product_id SERIAL references product(id),
    PRIMARY KEY (product_id, pharmacies_id)
);