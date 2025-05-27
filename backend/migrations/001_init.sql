-- Initial migration
CREATE TABLE audit_logs (
    id SERIAL PRIMARY KEY,
    action TEXT NOT NULL,
    user_name TEXT NOT NULL,
    timestamp TIMESTAMP DEFAULT NOW()
);
