pub mod downloader;

use specta::Type;
use rusqlite::{params, Connection, Result};

#[derive(Debug, Type, serde::Serialize)]
pub struct Docset {
    pub id: u16,
    pub name: String,
    pub alias: String,
    pub feed_url: String,
    pub docset_path: String,
    pub downloaded: bool,
}

#[derive(Type, serde::Serialize)]
pub struct SearchIndex {
    pub id: u16,
    pub name: String,
    pub doctype: String,
    pub html_path: String,
    pub docset_name: String,
}

pub fn docsets_base_path() -> String {
    "/workspaces/docbrowser/docsets".to_string() // TODO: pathはconfigからとってくる
}

pub fn docsets_master_db_path() -> String {
    return format!("{}/docsets.sqlite3", docsets_base_path());
}

pub fn docset_path(docset_name: impl Into<String> + std::fmt::Display) -> String {
    return format!("{}/{}.docset", docsets_base_path(), docset_name);
}

pub fn docset_db_path(docset_name: impl Into<String> + std::fmt::Display) -> String {
    return format!(
        "{}/Contents/Resources/docSet.dsidx",
        docset_path(docset_name)
    );
}

pub fn open_my_db(db_path: &str) -> Result<Connection, rusqlite::Error> {
    let con = Connection::open(&db_path)?;
    Ok(con)
}



pub fn search_docsets(con: &Connection, word: &str) -> Vec<Docset> {
    // let query = format!("select id, name, alias, feed_url, docset_path from docsets where name like '%{}%'", word);
    let query = format!("select * from docsets where name like '%{}%'", word);
    let mut stmt = con.prepare(&query.to_string()).unwrap();
    let docset_results = stmt
        .query_map(params![], |row| {
            Ok(Docset {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                alias: row.get(2).unwrap(),
                feed_url: row.get(3).unwrap(),
                docset_path: row.get(4).unwrap(),
                downloaded: row.get(5).unwrap(),
            })
        })
        .unwrap();

    let mut _docsets: Vec<Docset> = Vec::new();
    for d in docset_results {
        let docset = d.unwrap();
        // println!("{:?}", docset);
        _docsets.push(docset);
    }
    _docsets
}

pub fn search_index(con: &Connection, word: &str, docset: Docset) -> Vec<SearchIndex> {
    let query = format!(
        "select id, name, type, path from searchIndex where name like '%{}%'",
        word
    );
    let mut stmt = con.prepare(&query.to_string()).unwrap();
    let search_indices = stmt
        .query_map(params![], |row| {
            Ok(SearchIndex {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                doctype: row.get(2).unwrap(),
                html_path: row.get(3).unwrap(),
                docset_name: docset.name.clone(),
            })
        })
        .unwrap();

    let mut _results: Vec<SearchIndex> = Vec::new();
    for p in search_indices {
        let index = p.unwrap();
        // println!("{:?}", docset);
        _results.push(index);
    }
    _results
}
