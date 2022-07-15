-- For installing necessary uuid functions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS tsm_system_rows ;

CREATE TYPE translation AS (
    id uuid,
    value varchar(255)
);

CREATE TYPE word_kind AS ENUM (
    'NONE',
    'NOUN',
    'PRONOUN',
    'VERB',
    'ADJECTIVE',
    'ADVERB',
    'PREPOSITION',
    'CONJUNCTION',
    'INTERJECTION',
    'DETERMINER',
    'OTHER'
);

CREATE TABLE words (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    word varchar(255) NOT NULL,

    kind word_kind NOT NULL DEFAULT 'NONE',
    tags text[] NOT NULL DEFAULT '{}',
    translations translation[] NOT NULL DEFAULT '{}',

    created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE folders (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name varchar(255) NOT NULL,

    parent uuid,

    -- TODO: create foreign key constraint when supported
    words uuid[] NOT NULL DEFAULT '{}', 

    created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT fk_parent 
        FOREIGN KEY(parent) 
        REFERENCES folders(id) 
        ON DELETE CASCADE
);