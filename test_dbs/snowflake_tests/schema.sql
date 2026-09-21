CREATE TABLE user_events (
    event_id INT,
    user_email VARCHAR COMMENT '@PII',
    payload VARIANT
);
