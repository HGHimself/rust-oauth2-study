#[macro_export]
macro_rules! shopify {
    ($config:expr, $db:expr) => {
        shopify_route::shopify_install($config, $db)
            .and_then(shopify_handler::shopify_install)
            .or(shopify_route::shopify_confirm($config, $db)
                .and_then(shopify_handler::shopify_confirm))
    };
}
