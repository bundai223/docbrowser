use rspc::Type;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Docset{
    id: u16,
    department:String,
    pub name: String,
    salary: u32,
    avg_salary:Option<f32>
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

pub fn docsets(con:&Connection) -> Vec<Docset> {
    let mut stmt = con.prepare("select id,department,name,salary from person").unwrap();
    let persons = stmt.query_map(params![], |row| {
      Ok(Docset {
          id: row.get(0).unwrap(),
          department: row.get(1).unwrap(),
          name: row.get(2).unwrap(),
          salary: row.get(3).unwrap(),
          avg_salary:None
      })
    }).unwrap();

    let mut _docsets: Vec<Docset> = Vec::new();
    for p in persons {
      let docset = p.unwrap();
      // println!("{:?}", docset);
      _docsets.push(docset);
    }
    _docsets
}

pub fn search_docsets(con:&Connection, word:&str) -> Vec<Docset> {
    let query = format!("select id,department,name,salary from person where name like '%{}%'", word);
    let mut stmt = con.prepare(&query.to_string()).unwrap();
    let persons = stmt.query_map(params![], |row| {
      Ok(Docset {
          id: row.get(0).unwrap(),
          department: row.get(1).unwrap(),
          name: row.get(2).unwrap(),
          salary: row.get(3).unwrap(),
          avg_salary:None
      })
    }).unwrap();

    let mut _docsets: Vec<Docset> = Vec::new();
    for p in persons {
      let docset = p.unwrap();
      // println!("{:?}", docset);
      _docsets.push(docset);
    }
    _docsets
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
          docset_name: docset_name
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

pub fn select_window(con:&Connection) {
    let mut stmt = con.prepare("select id,department,name,salary,avg(salary) over defw from person window defw as (partition by department)").unwrap();
    let persons = stmt.query_map(params![], |row| {
      Ok(Docset {
          id: row.get_unwrap(0),
          department: row.get_unwrap(1),
          name: row.get_unwrap(2),
          salary: row.get_unwrap(3),
          avg_salary:Some(row.get_unwrap(4))
      })
    }).unwrap();

    for p in persons {
      println!("{:?}", p.unwrap());
    }
}
