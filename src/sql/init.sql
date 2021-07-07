CREATE TABLE post_publications(
    id INTEGER PRIMARY KEY,
    body TEXT,
    title TEXT,
    mineature_url TEXT
);
CREATE TABLE admins_ips(
    id INTEGER PRIMARY KEY,
    ip VARCHAR(64)
);