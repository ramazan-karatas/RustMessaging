-- USERS
CREATE TABLE IF NOT EXISTS users (
                                     id UUID PRIMARY KEY,
                                     username TEXT NOT NULL UNIQUE,
                                     created_at TIMESTAMPTZ NOT NULL DEFAULT now()
    );

-- MESSAGES
CREATE TABLE IF NOT EXISTS messages (
                                        id UUID PRIMARY KEY,
                                        from_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    to_user_id   UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    read_at TIMESTAMPTZ NULL
    );

-- Indexes for inbox/sent queries
CREATE INDEX IF NOT EXISTS idx_messages_to_created_at
    ON messages (to_user_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_messages_from_created_at
    ON messages (from_user_id, created_at DESC);
