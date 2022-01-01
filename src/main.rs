mod handler;

use actix_web::{App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handler::create_todo)
            .service(handler::list_todo)
            .service(handler::update_todo)
            .service(handler::delete_todo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
