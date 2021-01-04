CREATE TABLE mailing_list_subscribers (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    company VARCHAR NOT NULL,
    interest TEXT NOT NULL,
    wants_podcast_updates BOOLEAN NOT NULL DEFAULT 'f',
    wants_newsletter BOOLEAN NOT NULL DEFAULT 'f',
    wants_product_updates BOOLEAN NOT NULL DEFAULT 'f',
    date_added TIMESTAMPTZ NOT NULL,
    date_optin TIMESTAMPTZ NOT NULL,
    date_last_changed TIMESTAMPTZ NOT NULL,
    notes TEXT NOT NULL,
    tags TEXT [] NOT NULL,
    link_to_people TEXT [] NOT NULL,
    airtable_record_id VARCHAR NOT NULL
)
