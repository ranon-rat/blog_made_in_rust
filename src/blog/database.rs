#[allow(unused_variables)]
#[allow(dead_code)]
use crate::blog::types::{PublicationPost, QueryPublication};

use markdown::to_html;
use rusqlite::{params, Connection};
/*
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
*/
fn get_connection() -> Connection {
    let conn = Connection::open("src/sql/database.db").unwrap();
    conn
}
pub fn insert_publication(publication: &PublicationPost) -> usize {
    let sql = "INSERT INTO post_publications(body,title,mineature_url) 
    VALUES(?1,?2,?3)
    ";
    let database = get_connection();
    database
        .execute(
            sql,
            params![
                to_html(&publication.body),
                publication.title,
                publication.mineature_url
            ],
        )
        .unwrap_or(0)
}

/*
CREATE TABLE post_publications(
    id INTEGER PRIMARY KEY,
    title TEXT,
    mineature_url TEXT
    body TEXT,
);
*/
pub fn get_publication(publ: QueryPublication) -> PublicationPost {
    let sql = "SELECT * FROM post_publications WHERE id=?1 ";
    let database = get_connection();
    database
        .query_row(sql, params![publ.id], |r| {
            Ok(PublicationPost {
                title: (r.get(1).unwrap_or(String::from("404"))),
                mineature_url: (r.get(2).unwrap_or(String::from("404"))),
                body: (r.get(3).unwrap_or(String::from("sorry"))),
            })
        })
        .unwrap_or(PublicationPost {
            title: (String::from("404")),
            mineature_url: (String::from("404")),
            body: (String::from("sorry")),
        })
}
