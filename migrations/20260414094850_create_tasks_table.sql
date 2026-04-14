CREATE TABLE tasks (
    id            UUID        PRIMARY KEY DEFAULT gen_random_uuid(), 
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title         TEXT        NOT NULL,
    description         TEXT  ,
    priority            TEXT        NOT NULL,
    status  TEXT     NOT NULL,
    due_date TIMESTAMPTZ,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);