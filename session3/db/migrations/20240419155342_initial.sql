-- Add migration script here
-- Drop the messages table if it exists
DROP TABLE IF EXISTS messages;

-- Create the messages table
CREATE TABLE messages (
    id INTEGER PRIMARY KEY NOT NULL,
    message TEXT NOT NULL
);

-- Insert three rows into the messages table with specified IDs
INSERT INTO messages (id, message) VALUES (1, 'Hello, World!');
INSERT INTO messages (id, message) VALUES (2, 'Hello, Galaxy!');
INSERT INTO messages (id, message) VALUES (3, 'Hello, Universe!');
