CREATE TABLE user_sessions (
   id SERIAL PRIMARY KEY,
    public_address VARCHAR(255) NOT NULL,
    session_token VARCHAR(255)  NOT NULL, 
     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);