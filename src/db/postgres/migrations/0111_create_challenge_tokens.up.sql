
CREATE TABLE challenge_tokens (
     id SERIAL PRIMARY KEY,
    public_address VARCHAR(255) NOT NULL  ,
    challenge TEXT NOT NULL,
      created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);