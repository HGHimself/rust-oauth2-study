use rust_oauth2_study::{
    config::Config,
    db_conn::DbConn,
    db_test_url,
    handlers::{hello_handler, shopify_handler},
    models::shopify_connection::{ShopifyConnection, read},
    routes::{hello_route, shopify_route},
    schema::shopifyConnection,
};
use diesel::prelude::*;
use dotenv::dotenv;
use std::sync::Arc;
use warp::{self, Filter};

fn cleanup_table(conn: &PgConnection) {
    diesel::delete(shopifyConnection::table)
        .execute(conn)
        .unwrap();
}

#[tokio::test]
async fn it_inserts_on_shopify_installation() {
    let test_db_url = db_test_url();
    let config = Arc::new(Config::new());
    let db_conn = Arc::new(DbConn::new(&test_db_url));
    let shopify = shopify_route::shopify_install(config.clone(), db_conn.clone())
        .and_then(shopify_handler::shopify_install)
        .with(warp::log("shopify"));

    // send the request to our api,
    // hopefully sending back a redirect and saving an instance in the db
    let res = warp::test::request()
        .method("GET")
        .path(
            "/shopify_install?\
            hmac=6bce34e3ef95e442619456f4243fd785a7c25a182f3657018bef4737043bcf84\
            &shop=bdrocketstore.myshopify.com\
            &timestamp=1623154978",
        )
        .reply(&shopify)
        .await;
    assert_eq!(res.status(), 301);

    let shopify_connection = read(&db_conn.get_conn());
    assert!(0 < shopify_connection.len());

    let my_shopify_connection = shopify_connection
        .iter()
        .find(|&x| x.shop == "bdrocketstore.myshopify.com");
    assert!(
        my_shopify_connection.is_some(),
        "Could not find the created shopify_connection in the database!"
    );

    cleanup_table(&db_conn.get_conn());
}

// shopify_route::shopify_confirm(config.clone(), db_conn.clone())
//     .and_then(shopify_handler::shopify_confirm),
