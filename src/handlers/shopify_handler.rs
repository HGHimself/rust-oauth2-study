use crate::{
    config::Config, db_conn::DbConn, models::shopify_connection, services::shopify_service,
    utils::gen_uuid, AccessTokenResponse, ConfirmQueryParams, InstallQueryParams,
};
use reqwest::Client;
use std::sync::Arc;
use warp::{self, http::Uri};

// when shopkeep requests to install our app,
// they will click a link taking them to this handler.
//
// We redirect them back to their store's domain
// to request access to x,y,z scope/permissions.
//
// e.x. https://{shop}.myshopify.com/admin/oauth/authorize
//          ?client_id={api_key}
//          &scope={scopes}
//          &redirect_uri={redirect_uri}
//          &state={nonce}
//          &grant_options[]={access_mode}
pub async fn shopify_install(
    params: InstallQueryParams,
    config: Arc<Config>,
    db_conn: Arc<DbConn>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let nonce = gen_uuid();
    let conn = &db_conn.get_conn();

    println!("{:?}", nonce);
    // save install request in db to verify later
    shopify_connection::NewShopifyConnection::new(params.shop.clone(), nonce.clone()).insert(conn);

    // uri for the conform install page
    let formatted_uri = format!(
        "https://{}/admin/oauth/authorize?client_id={}&scope={}&redirect_uri={}&state={}",
        params.shop,
        config.shopify_api_key,
        "read_orders,write_orders", // probably want to be config
        "https://localhost:3030/shopify_confirm", // probably want to be config
        nonce,
    );

    Ok(warp::redirect(
        String::from(formatted_uri).parse::<Uri>().unwrap(),
    ))
}

// https://example.org/some/redirect/uri?code={authorization_code}&hmac=da9d83c171400a41f8db91a950508985&host={base64_encoded_hostname}&timestamp=1409617544&state={nonce}&shop={shop_origin}
// POST https://{shop}.myshopify.com/admin/oauth/access_token
pub async fn shopify_confirm(
    params: ConfirmQueryParams,
    config: Arc<Config>,
    db_conn: Arc<DbConn>,
    client: Arc<Client>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = db_conn.get_conn();
    // try and find the shop without the completed request
    let shoption =
        shopify_connection::read_by_shop_and_nonce(&conn, params.shop.clone(), params.state);

    let shop_conn = if let Some(o) = shoption.get(0) {
        o
    } else {
        panic!("We are panicking here")
    };

    let form_body = form_body_from_args(
        config.shopify_api_key.clone(),
        config.shopify_api_secret.clone(),
        params.code,
    );

    let uri = if config.is_mocking {
        config.shopify_api_uri.clone()
    } else {
        format!("{}{}", config.shopify_api_uri.clone(), params.shop)
    };

    let access_token_json = shopify_service::get_access_token(client.clone(), form_body, uri)
        .await
        .expect("Could not fetch access token!");

    // update the shop here
    shopify_connection::update_access_token(&conn, &shop_conn, access_token_json.access_token)
        .expect("Could not insert to db");

    // gotta figure out the reply later
    Ok(warp::redirect(String::from("/").parse::<Uri>().unwrap()))
}

fn form_body_from_args(api_key: String, api_secret: String, code: String) -> Vec<(String, String)> {
    vec![
        (String::from("client_id"), api_key),
        (String::from("client_secret"), api_secret),
        (String::from("code"), code),
    ]
}
