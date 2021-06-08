use crate::{
    config::Config, db_conn::DbConn, models::shopify_connection, ConfirmQueryParams,
    InstallQueryParams,
};
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
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params);
    /*
        The nonce is the same one that your app provided to Shopify during step two.
        The hmac is valid. The HMAC is signed by Shopify as explained below, in Verification.
        The shop parameter is a valid shop hostname, ends with myshopify.com, and doesn't contain characters other than letters (a-z), numbers (0-9), dots, and hyphens.
    */

    let shop_results =
        shopify_connection::read_by_shop_and_nonce(&db_conn.get_conn(), params.shop, params.state);
    let shop = shop_results
        .get(0)
        .expect("No available shopify connection found");

    // gotta figure out the reply later
    Ok(warp::redirect(String::from("/").parse::<Uri>().unwrap()))
}
