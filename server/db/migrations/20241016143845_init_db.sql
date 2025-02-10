-- Db init

CREATE TYPE objectType AS ENUM ('dir', 'file');
CREATE TYPE userRoleType AS ENUM ('superuser', 'admin', 'user');

CREATE TABLE "User" (
    id UUID PRIMARY KEY,
    name_1 VARCHAR(100) NOT NULL,
    name_2 VARCHAR(100),
    name_3 VARCHAR(100),
    email VARCHAR(255) UNIQUE NOT NULL,
    hash_password VARCHAR(255) NOT NULL,
    role_type userRoleType NOT NULL,
    created_at timestamp without time zone NOT NULL default now(),
    updated_at timestamp without time zone,
    is_deleted BOOLEAN DEFAULT FALSE,
    deleted_at timestamp without time zone,
    is_blocked BOOLEAN DEFAULT FALSE,
    blocked_at timestamp without time zone,
    storage_size BIGINT default 0
);

CREATE TABLE "Object" (
    id UUID PRIMARY KEY,
    parent_id UUID REFERENCES "Object"(id),
    owner_id UUID REFERENCES "User"(id) NOT NULL,
    creator_id UUID REFERENCES "User"(id) NOT NULL,
    name VARCHAR(255) NOT NULL,
    size BIGINT,
    type objectType NOT NULL,
    mimetype VARCHAR(100),
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_at timestamp without time zone, 
    in_trash BOOLEAN DEFAULT FALSE,
    eliminated BOOLEAN DEFAULT FALSE
);

CREATE TABLE "LastSeen" (
    user_id UUID NOT NULL REFERENCES "User"(id),
    object_id UUID NOT NULL REFERENCES "Object"(id) ON DELETE CASCADE,
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, object_id)
);

CREATE TABLE "UserXObject" (
    user_id UUID NOT NULL REFERENCES "User"(id),
    object_id UUID NOT NULL REFERENCES "Object"(id) ON DELETE CASCADE,
    can_read BOOLEAN DEFAULT TRUE,
    can_edit BOOLEAN DEFAULT FALSE,
    can_delete BOOLEAN DEFAULT FALSE,
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_at timestamp without time zone,
    PRIMARY KEY (user_id, object_id)
);

CREATE TABLE "FavoriteObject" (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES "User"(id),
    object_id UUID NOT NULL REFERENCES "Object"(id) ON DELETE CASCADE,
    created_at timestamp without time zone NOT NULL DEFAULT now()
);

ALTER TABLE "Object"
    ADD CONSTRAINT fk_parent_id FOREIGN KEY (parent_id) REFERENCES "Object"(id) ON DELETE SET NULL;
