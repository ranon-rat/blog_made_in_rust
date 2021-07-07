use crate::blog::types::PublicationPost;

use markdown::to_html;
use rusqlite::{params, Connection};

fn get_connection() -> Connection {
    let conn = Connection::open("src/sql/database.db").unwrap();
    conn
}
pub fn insert_publication(publication:& PublicationPost) -> usize {
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
        .unwrap()
}
