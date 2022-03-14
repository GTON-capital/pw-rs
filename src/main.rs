use actix_web::{get, web, App, HttpServer, Responder};


mod client;
mod rpc;

use crate::client::{Client, Props};
use crate::rpc::{
    get_wftm_price,
};


#[tokio::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let endpoint = match std::env::var("RPC") {
        Ok(v) => v,
        _ => String::from("https://rpcapi-tracing.fantom.network")
    };
    let app_data = Client::new(Props { node_rpc: endpoint }).await;

    HttpServer::new(move || {
        let app = App::new()
            .app_data(web::Data::new(app_data.clone()))
            .service(get_wftm_price);

        app
    })
    .bind(("0.0.0.0", 8881))?
    .run()
    .await
}