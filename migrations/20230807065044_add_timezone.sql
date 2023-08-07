-- Add migration script here

-- Remove all data from the table
DELETE FROM ipfs_image;

-- Alter the table to change the data type of the time_created and updated_date columns to TIMESTAMPTZ
ALTER TABLE ipfs_image
    ALTER COLUMN time_created TYPE TIMESTAMPTZ,
    ALTER COLUMN updated_date TYPE TIMESTAMPTZ;

-- Insert 4 rows into the table, one of which has a NULL value for the category column
INSERT INTO ipfs_image (image, ipfs_image_url, category)
VALUES
    ('image1.jpg', 'https://ipfs.io/ipfs/Qm...', 'category1'),
    ('image2.jpg', 'https://ipfs.io/ipfs/Qm...', 'category2'),
    ('image3.jpg', 'https://ipfs.io/ipfs/Qm...', 'category3'),
    ('image4.jpg', 'https://ipfs.io/ipfs/Qm...', NULL);
