
-- Delete all rows from the table
DELETE FROM ipfs_image;

-- Reset the SERIAL column counter to 1

-- Add the width column
ALTER TABLE ipfs_image
ADD COLUMN width INT NOT NULL CHECK (width >= 0);

-- Add the height column
ALTER TABLE ipfs_image
ADD COLUMN height INT NOT NULL CHECK (height >= 0);

-- Add the prompt column
ALTER TABLE ipfs_image
ADD COLUMN prompt TEXT;

-- Add the hash_id column
ALTER TABLE ipfs_image
ADD COLUMN hash_id TEXT NOT NULL;

-- Create a hash index on the hash_id column
CREATE INDEX hash_id_index ON ipfs_image USING hash (hash_id);

CREATE INDEX category_hash_index ON ipfs_image USING hash (category);
