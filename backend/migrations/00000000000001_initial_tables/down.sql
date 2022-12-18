DROP TABLE comments;
DROP TABLE documents;
DROP TABLE user_preferences;
DELETE FROM users WHERE username = 'admin';
ALTER TABLE tickets DROP COLUMN resolution;
DROP TABLE notes;
DROP TABLE tickets;
DROP TABLE contacts;
DROP TABLE users;