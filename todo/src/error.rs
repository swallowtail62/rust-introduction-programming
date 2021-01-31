use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {}

impl ResponseError for MyError {}
