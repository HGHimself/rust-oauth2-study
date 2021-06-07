use crate::{config::Config, InstallQueryParams};
use std::sync::Arc;
use warp::{self, http::Uri};

// https://{shop}.myshopify.com/admin/oauth/authorize?client_id={api_key}&scope={scopes}&redirect_uri={redirect_uri}&state={nonce}&grant_options[]={access_mode}
pub async fn shopify_install(
    params: InstallQueryParams,
    config: Arc<Config>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let formatted_path = format!(
        "https://{}.myshopify.com/admin/oauth/authorize?\
            client_id={}\
            &scope={}\
            &redirect_uri={}\
            &state={}",
        params.shop,
        config.shopify_api_key,
        "read_all_orders",
        "localhost:3030/shopify-confirm",
        "random-nonce",
    );

    // here we will need to save some information like nonce and user data I guess

    Ok(warp::redirect(
        String::from(formatted_path).parse::<Uri>().unwrap(),
    ))
}

/*
// https://example.org/some/redirect/uri?code={authorization_code}&hmac=da9d83c171400a41f8db91a950508985&host={base64_encoded_hostname}&timestamp=1409617544&state={nonce}&shop={shop_origin}
POST https://{shop}.myshopify.com/admin/oauth/access_token
pub async fn shopify_comfirm(
    code: String,
    hmac: String,
    host: String,
    timestamp: String,
    state: String,
    shop: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    /*
    The nonce is the same one that your app provided to Shopify during step two.
    The hmac is valid. The HMAC is signed by Shopify as explained below, in Verification.
    The shop parameter is a valid shop hostname, ends with myshopify.com, and doesn't contain characters other than letters (a-z), numbers (0-9), dots, and hyphens.
    */


    Ok(warp::redirect(
        String::from(formatted_path).parse::<Uri>().unwrap(),
    ))
}
*/
