mod todo;

use actix_web::{web, post, get, Result};
use self::todo::ToDo;

/// This function create a todo.
/// The response is result of create.
#[post("/todo/create")]
pub async fn create_todo(request: web::Json<todo::ToDo>) -> Result<String> {
    println!("{:?}", request);
    let content = format!("{}", request.content);
    let new_todo = ToDo::new(content);
    match ToDo::insert_db(&new_todo) {
        Ok(()) => println!("ToDo insert successed."),
        Err(e) => println!("ToDo insert failed. Detail:{}", e),
    }
    Ok(format!("todo: {:?}", new_todo))
}

/// This function show a todo.
/// The response is saved Todo.
#[get("/todo/list")]
pub async fn list_todo() -> Result<String> {
    println!("GET /todo/lost");
    let todos = todo::ToDo::new(String::from("new content"));
    Ok(format!("{:?}", todos))
}

/// This function modify a todo.
/// The response is result of update.
#[post("/todo/update")]
pub async fn update_todo(request: web::Json<todo::ToDo>) -> Result<String> {
    println!("{:?}", request);
    match ToDo::update_db(&request) {
        Ok(()) => println!("ToDo update successed."),
        Err(e) => println!("ToDo update failed. Detail:{}", e),
    }
    Ok(format!("todo: {:?}", request))
}


/// This function delete a todo.
/// The response is result of delete.
#[post("/todo/delete")]
pub async fn delete_todo(todo: web::Json<todo::ToDo>) -> Result<String> {
    println!("{:?}", todo);
    Ok(format!("todo: {:?}", todo))
}
