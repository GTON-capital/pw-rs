use actix_web::{web, App, HttpServer};

mod client;
mod rpc;

use crate::client::{Client, Props};
use crate::rpc::*;

#[tokio::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let endpoint = match std::env::var("RPC") {
        Ok(v) => v,
        _ => String::from("https://rpcapi-tracing.fantom.network"),
    };
    let app_data = Client::new(Props { node_rpc: endpoint }).await;

    HttpServer::new(move || {
        let app = App::new()
            .app_data(web::Data::new(app_data.clone()))
            .service(get_wftm_price)
            .service(get_wftm_gton_gc_pool_lp)
            .service(get_usdc_gton_gc_pool_lp)
            .service(get_ftm_gton_liq)
            .service(get_usdc_gton_liq)
            .service(get_ftm_gton_lp)
            .service(get_gc_pol)
            .service(get_pw_model_with_pol_mln)
            .service(get_gc_pw_current_peg_usd)
            .service(get_gc_pw_current_peg_ftm)
            .service(get_gton_usdc_price)
            .service(get_gton_wftm_price);

        app
    })
    .bind(("0.0.0.0", 8881))?
    .run()
    .await
}
