use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Docset{
    id: u16,
    department:String,
    pub name: String,
    salary: u32,
    avg_salary:Option<f32>
}

pub fn open_my_db(db_path: &str) -> Result<Connection, rusqlite::Error> {
    let con = Connection::open(&db_path)?;
    // println!("{}", con.is_autocommit());
    Ok(con)
}

pub fn select_all(con:&Connection){
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

    for p in persons {
      println!("{:?}", p.unwrap());
    }
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