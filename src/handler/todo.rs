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

    pub fn insert_db(content: &String, finished: bool) -> Result<()> {
        let conn = connect_to_db()?;
        conn.execute(
            "INSERT INTO todolist(content, finished) VALUES (?1, ?2)",
            params![content, finished as i32]
        )?;
        Ok(())
    }

    pub fn update_db(todo: &ToDo) -> Result<()> {
        let conn = connect_to_db()?;
        conn.execute(
            "UPDATE todolist set content = ?1, finished = ?2 WHERE id = ?3",
            params![todo.content, todo.finished as i32, todo.id]
        )?;
        Ok(())
    }

    pub fn list_db() -> Result<Vec<ToDo>> {
        let conn = connect_to_db()?;
        let mut stmt = conn.prepare("SELECT * FROM todolist")?;
        let res_iter = stmt.query_map([], |row| {
            Ok(ToDo {
                id: row.get(0)?,
                content: row.get(1)?,
                finished: row.get(2)?,
            })
        })?;

        let mut todos = Vec::new();
        for todo in res_iter {
            match todo {
                Ok(todo) => todos.push(todo),
                Err(e) => panic!("{:?}", e),
            }
        }
        Ok(todos)
    }

    pub fn delete_db(id: &i32) -> Result<()> {
        let conn = connect_to_db()?;
        conn.execute(
            "DELETE FROM todolist WHERE id = ?1",
            params![id]
        )?;
        Ok(())
    }
}

fn connect_to_db() -> Result<Connection> {
    Connection::open("./tododb.sqlite3")
}
