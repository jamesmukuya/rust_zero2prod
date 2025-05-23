use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[allow(dead_code)]
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!",&name)
}

async fn health_check() -> impl Responder{
    //todo!()
    HttpResponse::Ok().finish()
}

/*
The run fn is no longer binary endpoint hence no need for the pro-macros. It should be public
*/
pub async fn run() -> Result<(),std::io::Error>{
    HttpServer::new(|| {
        App::new()
        //.route("/",web::get().to(greet))
        //.route("/{name}", web::get().to(greet))
        .route("/health_check",web::get().to(health_check))
    })
    //.bind("127.0.0.1:8000")? ---this address already in use
    .bind(("127.0.0.1",8080))?
    .run()
    .await
    //println!("Hello, world!");
}