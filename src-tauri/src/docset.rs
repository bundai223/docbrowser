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