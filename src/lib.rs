//This is where our entire logic will be...in the library crate.


// Here we are bringing in actix web into scope.
use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

// async fn greet(req: HttpRequest) -> impl Responder {
// let name = req.match_info().get("name").unwrap_or("World");
// format!("Hello {}!", &name)
// }

// This is the handler signature and its asyc by default
async fn handler_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}
pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
    App::new()
    .route("/handler_check", web::get().to(handler_check))
    })
    .bind("127.0.0.1:8000")?
    .run();
    // No .await here!
    Ok(server)
    }
    

// #[tokio::main]
// pub fn run() -> Result<Server, std::io::Error> {
// let server = HttpServer::new(|| {
// App::new()
// .route("/handler_check", web::get().to(handler_check))})
// .bind("127.0.0.1:8000")?.run();
// Ok(server)
// }