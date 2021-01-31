use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error)
}

impl ResponseError for MyError {}
