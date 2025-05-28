use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;

#[allow(dead_code)]
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!",&name)
}

//health check responder
async fn health_check() -> impl Responder{
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData{
    name: String,
    email: String
}

//subscribe responder
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse{
    HttpResponse::Ok().finish()
}

/*
The run fn is no longer binary endpoint hence no need for the pro-macros. It should be public
We return `Server` on the happy path and we dropped the `async` keyword
We have no .await call, so it is not needed anymore.
*/
pub fn run(listener:TcpListener) -> Result<Server,std::io::Error>{
    let server = HttpServer::new(|| {
        App::new()
        .route("/",web::get().to(greet))
        .route("/{name}", web::get().to(greet))
        .route("/health_check",web::get().to(health_check))
        // A new entry in our routing table for POST /subscriptions requests
        .route("/subscriptions",web::post().to(subscribe))
    })
    //.bind("127.0.0.1:8000")? ---this address already in use
    //.bind(("127.0.0.1",8080))?
    //choosing a random port for the health check
    .listen(listener)?
    .run();

    //No .await here
    Ok(server)
    //println!("Hello, world!");
}