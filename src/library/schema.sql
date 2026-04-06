PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS document (
    id               INTEGER PRIMARY KEY NOT NULL,
    sha256           BLOB                NOT NULL,
    extension        TEXT                NOT NULL,
    page_count       INTEGER             NOT NULL,
    import_filename  TEXT                    NULL,
    imported_at      INTEGER             NOT NULL
) STRICT;

CREATE UNIQUE INDEX document_sha256 ON document(sha256);

CREATE TABLE IF NOT EXISTS piece (
    id               INTEGER PRIMARY KEY NOT NULL,
    title            TEXT                NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS score (
    id               INTEGER PRIMARY KEY NOT NULL,
    piece_id         INTEGER             NOT NULL REFERENCES piece(id),
    title            TEXT                NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS page (
    id               INTEGER PRIMARY KEY NOT NULL,
    score_id         INTEGER             NOT NULL REFERENCES score(id),
    width            INTEGER             NOT NULL,
    height           INTEGER             NOT NULL,
    order_index      INTEGER             NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS layer (
    id               INTEGER PRIMARY KEY NOT NULL,
    page_id          INTEGER             NOT NULL REFERENCES page(id),
    content_type     INTEGER             NOT NULL,
    order_index      INTEGER             NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS layer_document_page (
    id               INTEGER PRIMARY KEY NOT NULL,
    layer_id         INTEGER             NOT NULL REFERENCES layer(id),
    document_id      INTEGER             NOT NULL REFERENCES document(id),
    document_page_no INTEGER             NOT NULL
) STRICT;
