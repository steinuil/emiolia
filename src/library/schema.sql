CREATE TABLE IF NOT EXISTS piece (
    id               INTEGER PRIMARY KEY NOT NULL,
    title            TEXT                NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS piece_variant (
    id               INTEGER PRIMARY KEY NOT NULL,
    piece_id         INTEGER             NOT NULL,
    title            TEXT                NOT NULL,
    FOREIGN KEY (piece_id) REFERENCES piece(id)
) STRICT;

CREATE TABLE IF NOT EXISTS piece_page (
    id               INTEGER PRIMARY KEY NOT NULL,
    piece_variant_id INTEGER             NOT NULL,
    order_index      INTEGER             NOT NULL,
    FOREIGN KEY (piece_variant_id) REFERENCES piece_variant(id)
) STRICT;

CREATE TABLE IF NOT EXISTS source (
    id               INTEGER PRIMARY KEY NOT NULL,
    sha256           BLOB                NOT NULL,
    extension        TEXT                NOT NULL,
    page_count       INTEGER             NOT NULL,
    import_filename  TEXT                NULL,
    imported_at      INTEGER             NOT NULL
) STRICT;

CREATE UNIQUE INDEX file_sha256 ON File(sha256);
