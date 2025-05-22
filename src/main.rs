use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!",&name)
}

async fn health_check() -> impl Responder{
    //todo!()
    HttpResponse::Ok()
}

/*
UNABLE TO GET HTTP OK FOR THE URLS EXCEPT DEFAULT ROOT I.E /{name} and "/health_check"

*/
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
        .route("/",web::get().to(greet))
        .route("/{name}", web::get().to(greet))
        .route("/health_check",web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
    //println!("Hello, world!");
}
