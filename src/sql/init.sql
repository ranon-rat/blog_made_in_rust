CREATE TABLE post_publications(
    id INTEGER PRIMARY KEY,
    
    title TEXT,
    mineature_url TEXT,
    body TEXT
    
);
CREATE TABLE admins_ips(
    id INTEGER PRIMARY KEY,
    ip VARCHAR(64)
);