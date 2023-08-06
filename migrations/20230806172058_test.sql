CREATE TABLE your_table_name (
    id SERIAL PRIMARY KEY,
    image TEXT NOT NULL,
    time_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    ipfs_image_url TEXT NOT NULL,
    category VARCHAR,
    updated_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
