use actix_web::{get, web, App, HttpServer, Responder};



mod client;
mod rpc;

use crate::rpc::{get_rpc_services, greet};


#[tokio::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let services = get_rpc_services().await;

    HttpServer::new(|| {
        let app = App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }));


        app.service(greet)
        // services.into_iter().map(|x| { app.service(x) })

        // app.service(services[0])
    })
    .bind(("0.0.0.0", 8881))?
    .run()
    .await
}