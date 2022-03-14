use std::sync::Arc;

use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::dev::HttpServiceFactory;


use serde_derive::{Serialize, Deserialize};

use crate::client::{Client, Props};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub result: f64,
}


#[get("/rpc/base-price")]
pub async fn get_wftm_price(client: web::Data<Client>) -> impl Responder {
  let result = client.get_wftm_price().await;
  format!("{:?}", Response { result })
}


#[get("/rpc/owned/base-pool-lps")]
pub async fn get_wftm_gton_gc_pool_lp(client: web::Data<Client>) -> impl Responder {
  let client_l = Box::into_raw(Box::new(client));
  let x = unsafe { Box::from_raw(client_l) };
  let r = Box::leak(x).get_wftm_gton_gc_pool_lp().await;
  format!("{:}", r)
}