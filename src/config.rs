use std::env;

#[derive(Clone)]
pub struct Config {
    pub app_addr: String,
    pub shopify_api_key: String,
    pub tls: bool,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        let app_host = env::var("HOST").expect("HOST must be set");
        let app_port = env::var("PORT").expect("PORT must be set");

        let app_addr = format!("{}:{}", app_host, app_port);

        let shopify_api_key = env::var("API_KEY_SHOPIFY").expect("API_KEY_SHOPIFY must be set");

        let tls = env::var("ENABLE_TLS")
            .expect("ENABLE_TLS must be set")
            .parse()
            .expect("ENABLE_TLS must be a bool");

        let cert_path = if tls {
            Some(env::var("CERT_PATH").expect("CERT_PATH must be set"))
        } else {
            None
        };

        let key_path = if tls {
            Some(env::var("KEY_PATH").expect("KEY_PATH must be set"))
        } else {
            None
        };

        Config {
            app_addr,
            shopify_api_key,
            tls,
            cert_path,
            key_path,
        }
    }
}
