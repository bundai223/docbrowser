use rspc::Type;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Docset {
  id: u16,
  pub name: String,
  alias: String,
  feed_url: String,
  docset_path: String
}

#[derive(Type, serde::Serialize)]
pub struct SearchIndex {
  id: u16,
  pub name: String,
  pub doctype: String,
  pub htmlpath: String,
  pub docset_name: String
}

pub fn docsets_master_db_path() -> String {
  "../docsets/docsets.sqlite3".to_string()
}

pub fn docset_db_path(docset_name: impl Into<String>) -> String {
  "../docsets/TypeScript.docset/Contents/Resources/docSet.dsidx".to_string()
}

pub fn open_my_db(db_path: &str) -> Result<Connection, rusqlite::Error> {
    let con = Connection::open(&db_path)?;
    // println!("{}", con.is_autocommit());
    Ok(con)
}

pub fn docsets(con:&Connection) -> Vec<Docset> {
    let mut stmt = con.prepare("select id, name, alias, feed_url, docset_path from docsets").unwrap();
    let docset_results = stmt.query_map(params![], |row| {
      Ok(Docset {
          id: row.get(0).unwrap(),
          name: row.get(1).unwrap(),
          alias: row.get(2).unwrap(),
          feed_url: row.get(3).unwrap(),
          docset_path: row.get(4).unwrap()
      })
    }).unwrap();

    let mut _docsets: Vec<Docset> = Vec::new();
    for d in docset_results {
      let docset = d.unwrap();
      // println!("{:?}", docset);
      _docsets.push(docset);
    }
    _docsets
}

pub fn search_docsets(con: &Connection, word: &str) -> Vec<Docset> {
    let query = format!("select id, name, alias, feed_url, docset_path from docsets where name like '%{}%'", word);
    let mut stmt = con.prepare(&query.to_string()).unwrap();
    let docset_results = stmt.query_map(params![], |row| {
      Ok(Docset {
          id: row.get(0).unwrap(),
          name: row.get(1).unwrap(),
          alias: row.get(2).unwrap(),
          feed_url: row.get(3).unwrap(),
          docset_path: row.get(4).unwrap()
      })
    }).unwrap();

    let mut _docsets: Vec<Docset> = Vec::new();
    for d in docset_results {
      let docset = d.unwrap();
      // println!("{:?}", docset);
      _docsets.push(docset);
    }
    _docsets
}

pub fn search_index(con:&Connection, word:&str, docset: Docset) -> Vec<SearchIndex> {
    let query = format!("select id, name, type, path from searchIndex where name like '%{}%'", word);
    let mut stmt = con.prepare(&query.to_string()).unwrap();
    let search_indices = stmt.query_map(params![], |row| {
      Ok(SearchIndex {
          id: row.get(0).unwrap(),
          name: row.get(1).unwrap(),
          doctype: row.get(2).unwrap(),
          htmlpath: row.get(3).unwrap(),
          docset_name: docset.name.clone(),
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

// pub fn select_window(con:&Connection) {
//     let mut stmt = con.prepare("select id,department,name,salary,avg(salary) over defw from person window defw as (partition by department)").unwrap();
//     let persons = stmt.query_map(params![], |row| {
//       Ok(Docset {
//           id: row.get_unwrap(0),
//           department: row.get_unwrap(1),
//           name: row.get_unwrap(2),
//           salary: row.get_unwrap(3),
//           avg_salary:Some(row.get_unwrap(4))
//       })
//     }).unwrap();

//     for p in persons {
//       println!("{:?}", p.unwrap());
//     }
// }
