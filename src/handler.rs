mod todo;

use std::path::PathBuf;

use actix_web::{web, Responder, HttpResponse, HttpRequest, post, get, Result};
use actix_files::NamedFile;

pub async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("page").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

/// This function create a todo.
/// The response is result of create.
#[post("/todo/create")]
pub async fn create_todo(todo: web::Json<todo::ToDo>) -> Result<String> {
    println!("{:?}", todo);
    Ok(format!("todo: {:?}", todo))
}

/// This function show a todo.
/// The response is saved Todo.
#[get("/todo/list")]
pub async fn list_todo() -> impl Responder {
    HttpResponse::Ok().body("todo")
}

/// This function modify a todo.
/// The response is result of update.
#[post("/todo/update")]
pub async fn update_todo(request: String) -> impl Responder {
    HttpResponse::Ok().body(request)
}


/// This function delete a todo.
/// The response is result of delete.
#[post("/todo/delete")]
pub async fn delete_todo(request: String) -> impl Responder {
    HttpResponse::Ok().body(request)
}
