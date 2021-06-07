#[macro_export]
macro_rules! shopify {
    ($config:expr) => {
        shopify_route::shopify_install($config).and_then(shopify_handler::shopify_install)
    };
}
