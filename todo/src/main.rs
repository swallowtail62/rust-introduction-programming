use actix_web::{App, get, HttpResponse, HttpServer};

use crate::error::MyError;

mod error;

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let response_body = "Hello world!";
    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
