use serde::Serialize;
#[derive(Serialize,Debug)]

pub struct Publication{
    pub id:i64,
    pub body:String,
    pub title:String,
    pub mineature_url:String,
}
/*
CREATE TABLE post_publications(
    id INTEGER PRIMARY KEY,
    body TEXT,
    title TEXT,
    mineature_url TEXT,
);
*/