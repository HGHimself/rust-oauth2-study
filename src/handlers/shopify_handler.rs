use crate::{
    config::Config, db_conn::DbConn, models::shopify_connection, AccessTokenResponse,
    ConfirmQueryParams, InstallQueryParams,
};
use reqwest::Client;
use std::sync::Arc;
use warp::{self, http::Uri};

// https://{shop}.myshopify.com/admin/oauth/authorize?client_id={api_key}&scope={scopes}&redirect_uri={redirect_uri}&state={nonce}&grant_options[]={access_mode}
pub async fn shopify_install(
    params: InstallQueryParams,
    config: Arc<Config>,
    db_conn: Arc<DbConn>,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params);

    let formatted_path = format!(
        "https://{}/admin/oauth/authorize?\
            client_id={}\
            &scope={}\
            &redirect_uri={}\
            &state={}",
        params.shop.clone(),
        config.shopify_api_key,
        "read_orders,write_orders",
        "https://localhost:3030/shopify_confirm",
        "random-nonce",
    );

    shopify_connection::NewShopifyConnection::new(params.shop, String::from("random-nonce"))
        .insert(&db_conn.get_conn());

    Ok(warp::redirect(
        String::from(formatted_path).parse::<Uri>().unwrap(),
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
    // try and find the shop without the completed request
    let shop = shopify_connection::read_by_shop_and_nonce(
            &db_conn.get_conn(),
            params.shop.clone(),
            params.state,
        )
        .get(0)
        .expect("No available shopify connection found");

    let form_body = vec![
        (String::from("client_id"), config.shopify_api_key.clone()),
        (
            String::from("client_secret"),
            config.shopify_api_secret.clone(),
        ),
        (String::from("code"), params.code),
    ];

    let access_token_json = fetch_access_token(client, &form_body, params.shop.clone());

    // update the shop here

    // gotta figure out the reply later
    Ok(warp::redirect(String::from("/").parse::<Uri>().unwrap()))
}

pub async fn fetch_access_token(
    client: Arc<Client>,
    form_body: &Vec<(String, String)>,
    shop: String,
) -> reqwest::Result<AccessTokenResponse> {
    let access_token_json: AccessTokenResponse = client
        .post(format!("https://{}/admin/oauth/access_token", shop))
        .form(&form_body)
        .send()
        .await?
        .json()
        .await?;

    Ok(access_token_json)
}
