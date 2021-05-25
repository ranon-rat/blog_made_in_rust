use rusqlite::{params, Connection};
use markdown::to_html;
use crate::blog::types::Publication;
/*
CREATE TABLE post_publications(
    id INTEGER PRIMARY KEY,
    body TEXT,
    title TEXT,
    mineature_url TEXT,
);
CREATE TABLE admins_ips(
    id INTEGER PRIMARY KEY,
    ip VARCHAR(64)
);
*/
fn get_connection()->Connection {
    let conn = Connection::open("src/sql/database.db").unwrap();
    conn
  
}
pub fn insert_publication(publication:Publication){
    let sql="INSERT INTO post_publications(body,title,mineature_url) 
    VALUES(?1,?2,?3)
    ";
    let database=get_connection();
    database.execute(sql, params![
        to_html(&publication.body),
        publication.title,
        publication.mineature_url
    ]).unwrap();
}