CREATE TABLE users (
    id INT,
    email VARCHAR COMMENT '@PII'
);
CREATE TABLE events (
    id INT,
    dt DATE COMMENT '@PARTITION',
    user_id INT
);
