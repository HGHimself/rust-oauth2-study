pub mod view;

use log::info;
use ory_hydra_client::apis::{admin_api, configuration::Configuration};
use rust_oauth2_study::{LoginFormBody, LoginQueryParams}
use std::sync::Arc;
use tera::{Context, Tera};
use warp::{self, http::Uri, Filter};

#[tokio::main]
async fn main() {
    if ::std::env::var_os("RUST_LOG").is_none() {
        ::std::env::set_var("RUST_LOG", "warp=info,auth_svc=trace,api_access=trace");
    }
    env_logger::init();

    let routes = auth_routes();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

pub fn auth_routes() -> warp::filters::BoxedFilter<(impl warp::reply::Reply,)> {
    warp::path("login")
        .and(login_page().or(accept_login()))
        .boxed()
}

pub fn with_hydra_api() -> warp::filters::BoxedFilter<(Arc<Configuration>,)> {
    warp::any()
        .map(move || {
            let mut configuration = Configuration::new();
            configuration.base_path = "localhost:4444".to_owned();
            Arc::new(configuration)
        })
        .boxed()
}

pub fn accept_login() -> warp::filters::BoxedFilter<(impl warp::Reply,)> {

    warp::post()
        .and(
            warp::body::content_length_limit(1024 * 32)
                .and(warp::body::form())
                .and(with_hydra_api())
                .map(|form_body: LoginFormBody, hydra_api: Arc<Configuration>| {
                    // Add logic here to verify the username and password from the submitted login form

                    // Accepting login request, although you could still deny the login request if something else went wrong
                    match form_body
                        .login_challenge
                        .map(accept_login_from_challenge)
                        .unwrap_or_else(|| warp::redirect(String::from("/").parse::<Uri>().unwrap()))
                }),
        )
        .boxed()
}

pub async fn accept_login_from_challenge(login_challenge: String) -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
    info!("Accepting login request with Hydra");
    let completed_request = admin_api::accept_login_request(
        &hydra_api,
        &login_challenge,
        Some(ory_hydra_client::models::AcceptLoginRequest::new(
            // We are using a hardcoded subject here, the subject should be an immutable id of the user that is loggin in
            // to let Hydra know which user to associate with this login
            "hardcoded_subject".to_owned(),
        )),
    )
    .await
    .unwrap();

    // Redirecting to hydra
    warp::redirect(completed_request.redirect_to.parse::<Uri>().unwrap())
}

pub fn login_page() -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
    warp::get()
        .and(warp::query::query())
        .and(view::with_tera())
        .and(with_hydra_api())
                .map(
                    move |query_params: LoginQueryParams, tera: Tera, hydra_api: Arc<Configuration>| {

                        // The challenge is used to fetch information about the login request from ORY Hydra.
                        query_params
                            .login_challenge
                            .map(get_login)
                            .unwrap_or_else(|| {
                                let body = tera.render("login.html", &Context::new()).unwrap();
                                Box::new(warp::reply::html(body)) as Box<dyn warp::Reply>
                            })
                    },
                )
        .boxed()
}

pub async fn get_login(login_challenge: String) -> warp::filters::BoxedFilter<(impl warp::Reply,)>  {
    let login_request =
        admin_api::get_login_request(&hydra_api, &login_challenge).await.unwrap();

    // If ory_hydra_client was already able to authenticate the user, skip will be true and we do not need to re-authenticate
    if login_request.skip {
        info!("Hydra was already able to authenticate the user, skipping login as we do not need to re-authenticate");
        info!("Accepting login request with Hydra");

        // You can apply logic here, for example update the number of times the user logged in.
        // ...

        // Now it's time to grant the login request. You could also deny the request if something went terribly wrong
        // (e.g. your arch-enemy logging in...)
        let completed_request = admin_api::accept_login_request(
                &hydra_api,
                &login_challenge,
                Some(ory_hydra_client::models::AcceptLoginRequest::new(
                    // All we need to do is to confirm that we indeed want to log in the user.
                    // We are using a hardcoded subject here, the subject should be an immutable id of the user that is loggin in
                    // to let Hydra know which user to associate with this login.
                    "hardcoded_subject".to_owned(),
                )),
            )
            .await
            .unwrap();

        // All we need to do now is to redirect the user back to ory_hydra_client!
        Box::new(warp::redirect(
                completed_request
                    .redirect_to
                    .parse::<Uri>().unwrap()
            ,
        )) as Box<dyn warp::Reply>
    } else {
        // If authentication can't be skipped we MUST show the login UI.
        info!("Sending user to login");

        // The challenge will be a hidden input field
        let mut context = Context::new();
        context.insert("login_challenge", &login_challenge);

        let body = tera.render("login.html", &context).unwrap();
        Box::new(warp::reply::html(body)) as Box<dyn warp::Reply>
    }
}
