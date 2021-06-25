use rust_oauth2_study::{
    config::Config, db_conn::DbConn, handlers::shopify_handler, routes::shopify_route,
};
use std::net::SocketAddr;
use std::sync::Arc;
use warp::Filter;

pub mod api;

#[tokio::main]
async fn main() {
    let config = Arc::new(Config::new(false));
    let db_conn = Arc::new(DbConn::new(&config.db_path));
    let client = Arc::new(reqwest::Client::new());

    let shopify =
        shopify!(config.clone(), db_conn.clone(), client.clone()).with(warp::log("shopify"));

    let end = shopify;

    let socket_address = config
        .clone()
        .app_addr
        .parse::<SocketAddr>()
        .expect("Could not parse Addr");

    println!("Listening at {}", &config.app_addr);

    if config.clone().tls {
        println!("TLS Enabled!");

        warp::serve(end)
            .tls()
            .cert_path(config.clone().cert_path.as_ref().unwrap())
            .key_path(config.clone().key_path.as_ref().unwrap())
            .run(socket_address)
            .await;
    } else {
        warp::serve(end).run(socket_address).await;
    }
}
