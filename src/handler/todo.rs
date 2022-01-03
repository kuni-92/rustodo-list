use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ToDo {
    pub content: String,
    pub finished: bool,
}

impl ToDo {
    pub fn new(content: String) -> Self {
        ToDo{content, finished: false}
    }

    pub fn write_to_db(todo: &ToDo) -> Result<()> {
        let finished = if todo.finished {0} else {1};

        let conn = connect_to_db()?;
        conn.execute(
            "INSERT INTO todolist(content, finished) VALUES (?1, ?2)",
            params![todo.content, finished]
        )?;
        Ok(())
    }
}


fn connect_to_db() -> Result<Connection> {
    Connection::open("./tododb.sqlite3")
}
