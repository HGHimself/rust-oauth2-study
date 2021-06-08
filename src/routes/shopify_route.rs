use crate::{config::Config, with_config, ConfirmQueryParams, InstallQueryParams};
use std::sync::Arc;
use warp::{filters::BoxedFilter, Filter};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("shopify_install").boxed()
}

pub fn shopify_install(config: Arc<Config>) -> BoxedFilter<(InstallQueryParams, Arc<Config>)> {
    warp::get()
        .and(path_prefix())
        .and(warp::query::query::<InstallQueryParams>())
        .and(with_config(config))
        .boxed()
}

pub fn shopify_confirm(config: Arc<Config>) -> BoxedFilter<(ConfirmQueryParams, Arc<Config>)> {
    warp::get()
        .and(warp::path("shopify_confirm"))
        .and(warp::query::query::<ConfirmQueryParams>())
        .and(with_config(config))
        .boxed()
}
