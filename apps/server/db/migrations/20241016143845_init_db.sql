
-- db init
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,

    email TEXT NOT NULL,
    password TEXT NOT NULL,
    
    create_date timestamp with time zone default now(),
    storage_size bigint NOT NULL default 0
    );

CREATE TABLE IF NOT EXISTS objects (
    id UUID PRIMARY KEY,
    parent_id UUID NULL,
    FOREIGN KEY (parent_id) REFERENCES objects(id),

    name TEXT NOT NULL,
    size bigint NOT NULL,

    owner_id UUID NOT NULL,
    FOREIGN KEY (owner_id) REFERENCES users(id),

    create_date timestamp with time zone NOT NULL default now(),
    update_date timestamp with time zone NOT NULL default now()
);