use actix_web::{Responder, HttpResponse, post, get};

/// This function create a todo.
/// The response is result of create.
#[post("/todo/create")]
pub async fn create_todo(request: String) -> impl Responder {
    HttpResponse::Ok().body(request)
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
