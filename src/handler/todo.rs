use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ToDo {
    pub id: i32,
    pub content: String,
    pub finished: bool,
}

impl ToDo {
    pub fn new(content: String) -> Self {
        ToDo{id: 0, content, finished: false}
    }

    pub fn insert_db(todo: &ToDo) -> Result<()> {
        let finished = finished_to_i32(todo.finished);

        let conn = connect_to_db()?;
        conn.execute(
            "INSERT INTO todolist(content, finished) VALUES (?1, ?2)",
            params![todo.content, finished]
        )?;
        Ok(())
    }

    pub fn update_db(todo: &ToDo) -> Result<()> {
        let finished = finished_to_i32(todo.finished);

        let conn = connect_to_db()?;
        conn.execute(
            "UPDATE todolist set content = ?1, finished = ?2 WHERE id = ?3",
            params![todo.content, finished, todo.id]
        )?;
        Ok(())
    }
}

fn finished_to_i32(finished: bool) -> i32 {
    if finished {0} else {1}
}

fn connect_to_db() -> Result<Connection> {
    Connection::open("./tododb.sqlite3")
}
