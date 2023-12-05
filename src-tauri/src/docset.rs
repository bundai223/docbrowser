use std::fs;

use rspc::Type;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Docset{
    id: u16,
    pub name: String,
}

#[derive(Type, serde::Serialize)]
pub struct SearchIndex {
  id: u16,
  pub name: String,
  pub doctype: String,
  pub html_path: String,
  pub docset_name: String,
}

pub fn docset_path(docset_name: &str) -> String {
    let docset_base_path = "./../docsets";
    let p = format!("{}/{}.docset", docset_base_path, docset_name);

    return p;
}

pub fn docset_database_path(docset_name: &str) -> String {
    let sqlite_file_path = "Contents/Resources/docSet.dsidx";
    let p = format!("{}/{}", docset_path(docset_name), sqlite_file_path);

    return p;
}

pub fn document_page_path(docset_name: &str, rel_path: &str) -> String {
    let p = format!("{}/Contents/Resources/Documents/{}", docset_path(docset_name), rel_path);

    return p;
}

pub fn open_my_db(db_path: &str) -> Result<Connection, rusqlite::Error> {
    let con = Connection::open(&db_path)?;
    Ok(con)
}

pub fn search_index(con:&Connection, docset_name: &str, word:&str) -> Vec<SearchIndex> {
    let query = format!("select id, name, type, path from searchIndex where name like '%{}%'", word);
    let mut stmt = con.prepare(&query.to_string()).unwrap();
    let search_indices = stmt.query_map(params![], |row| {
      Ok(SearchIndex {
          id: row.get(0).unwrap(),
          name: row.get(1).unwrap(),
          doctype: row.get(2).unwrap(),
          html_path: row.get(3).unwrap(),
          docset_name: docset_name.to_string()
      })
    }).unwrap();

    let mut _results: Vec<SearchIndex> = Vec::new();
    for p in search_indices {
      let index = p.unwrap();
      // println!("{:?}", docset);
      _results.push(index);
    }
    _results
}

pub fn read_html(docset_name: &str, rel_path: &str) -> String {
  let path = document_page_path(docset_name, rel_path);

  println!("doc page path: {}", path);
  let websites = fs::read_to_string(path).unwrap();
  return websites;
}