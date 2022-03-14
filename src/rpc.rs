// use std::lazy::Lazy;

use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::dev::HttpServiceFactory;


#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}





pub async fn get_rpc_services() -> Vec<impl HttpServiceFactory> {
  // vec![
  //   greet
  // ]

  let mut r = vec![];
  r.push(greet);
  r
}