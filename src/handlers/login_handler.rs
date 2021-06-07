use crate::{LoginFormBody, LoginQueryParams};
use log::info;
use ory_hydra_client::apis::{admin_api, configuration::Configuration};
use std::sync::Arc;
use tera::{Context, Tera};
use warp::{self, http::Uri, Filter};

pub async fn login(
    query_params: LoginQueryParams,
    tera: Tera,
    hydra_api: Arc<Configuration>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(login_challenge) = query_params.login_challenge {
        println!("Trying to login! getting request from api");
        let login_request = admin_api::get_login_request(&hydra_api, &login_challenge)
            .await
            .unwrap();

        println!("{:?}", login_request);

        // If ory_hydra_client was already able to authenticate the user, skip will be true and we do not need to re-authenticate
        if login_request.skip {
            println!("Hydra was already able to authenticate the user, skipping login as we do not need to re-authenticate");
            println!("Accepting login request with Hydra");

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
            // Ok(warp::redirect(
            //     completed_request.redirect_to.parse::<Uri>().unwrap(),
            // ))
            Ok(warp::reply::html(String::from("Login Completed!")))
        } else {
            // If authentication can't be skipped we MUST show the login UI.
            println!("Sending user to login");

            // The challenge will be a hidden input field
            let mut context = Context::new();
            context.insert("login_challenge", &login_challenge);

            let body = tera.render("login.html", &context).unwrap();
            Ok(warp::reply::html(body))
        }
    } else {
        let mut context = Context::new();
        context.insert("login_challenge", "12345");

        let body = tera.render("login.html", &context).unwrap();
        Ok(warp::reply::html(body))
    }
}

pub async fn accept_login(
    form_body: LoginFormBody,
    hydra_api: Arc<Configuration>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match form_body.login_challenge {
        Some(login_challenge) => {
            println!("Accepting login request with Hydra");
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
            Ok(warp::redirect(
                completed_request.redirect_to.parse::<Uri>().unwrap(),
            ))
        }
        None => Ok(warp::redirect(String::from("/").parse::<Uri>().unwrap())),
    }
}
