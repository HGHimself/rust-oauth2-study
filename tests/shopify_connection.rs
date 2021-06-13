mod shopify_integration_tests {

    use diesel::prelude::*;
    use dotenv::dotenv;
    use mockito::mock;
    use mocktopus::mocking::*;
    use rust_oauth2_study::{
        config::Config,
        db_conn::DbConn,
        db_test_url,
        handlers::{hello_handler, shopify_handler},
        models::shopify_connection::{
            create, read, read_by_shop, read_by_shop_and_nonce, NewShopifyConnection,
            ShopifyConnection,
        },
        routes::{hello_route, shopify_route},
        schema::shopify_connections,
        utils::gen_uuid,
        AccessTokenResponse,
    };
    use std::sync::Arc;
    use uuid::Uuid;
    use warp::{self, Filter};

    fn cleanup_table(conn: &PgConnection) {
        diesel::delete(shopify_connections::table)
            .execute(conn)
            .unwrap();
    }

    #[tokio::test]
    async fn it_inserts_on_shopify_installation() {
        let config = Arc::new(Config::new(false));
        let db_conn = Arc::new(DbConn::new(&db_test_url()));
        let client = Arc::new(reqwest::Client::new());

        let shopify = shopify_route::shopify_install(config.clone(), db_conn.clone())
            .and_then(shopify_handler::shopify_install)
            .with(warp::log("shopify"));

        let shop_name = "bestbudz.myshopify.com";
        let nonce = "some-nonce";

        gen_uuid.mock_safe(move || MockResult::Return(nonce.to_string()));

        // send the request to our api,
        // hopefully sending back a redirect and saving an instance in the db
        let res = warp::test::request()
            .method("GET")
            .path(&format!(
                "/shopify_install\
                ?hmac=00a329c0648769a73afac7f9381e08fb43dbea72\
                &shop={}\
                &timestamp=1623154978",
                shop_name
            ))
            .reply(&shopify)
            .await;
        assert_eq!(res.status(), 301);

        let shopify_connection = read_by_shop_and_nonce(
            &db_conn.get_conn(),
            shop_name.to_string(),
            nonce.to_string(),
        );
        assert!(0 < shopify_connection.len());

        let my_shopify_connection = shopify_connection.iter().find(|&x| x.shop == shop_name);
        assert!(
            my_shopify_connection.is_some(),
            "Could not find the created shopify_connection in the database!"
        );

        cleanup_table(&db_conn.get_conn());
    }

    #[tokio::test]
    async fn it_follows_shopify_confirm_flow() {
        let test_db_url = db_test_url();

        let mut config = Config::new(true);
        config.set_shopify_api_uri(mockito::server_url());
        let arc_config = Arc::new(config);

        let db_conn = Arc::new(DbConn::new(&test_db_url));
        let client = Arc::new(reqwest::Client::new());
        let shopify =
            shopify_route::shopify_confirm(arc_config.clone(), db_conn.clone(), client.clone())
                .and_then(shopify_handler::shopify_confirm)
                .with(warp::log("shopify"));

        let shop_name = "bestbudz.myshopify.com";
        let nonce = "00a329c0648769a73afac7f9381e08fb43dbea72";
        let access_token = "f85632530bf277ec9ac6f649fc327f17";

        let new_shopify_connection =
            NewShopifyConnection::new(shop_name.to_string(), nonce.to_string());
        new_shopify_connection.insert(&db_conn.get_conn());

        let _m = mock("POST", "/admin/oauth/access_token")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(&format!(
                "{{\"access_token\": \"{}\",\"scope\": \"write_orders,read_customers\"}}",
                access_token
            ))
            .create();

        let res = warp::test::request()
            .method("GET")
            .path(&format!(
                "/shopify_confirm\
                    ?code=e58c4d79044bccd3d9918d6608b09cad\
                    &hmac=c9310602e83ddcb33ec3b1418d02f82c51c6af59c7e5a93953c6c216986e0ffb\
                    &host=YmRyb2NrZXRzdG9yZS5teXNob3BpZnkuY29tL2FkbWlu\
                    &shop={}\
                    &state={}\
                    &timestamp=1623437117",
                shop_name, nonce
            ))
            .reply(&shopify)
            .await;

        let shopify_connections = read_by_shop(&db_conn.get_conn(), shop_name.to_string());

        assert_eq!(1, shopify_connections.len());
        let my_shopify_connection = shopify_connections.iter().find(|x| x.shop == shop_name);
        assert!(
            my_shopify_connection.is_some(),
            "Could not find the created shopify_connection in the database!"
        );

        println!("{:?}", my_shopify_connection);

        assert_eq!(
            my_shopify_connection
                .unwrap()
                .access_token
                .as_ref()
                .unwrap(),
            access_token
        );

        cleanup_table(&db_conn.get_conn());
    }
}
