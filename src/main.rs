use rust_oauth2_study::{
    handlers::{hello_handler, login_handler},
    routes::{hello_route, login_route},
    LoginQueryParams,
};
use warp::Filter;

pub mod api;

#[tokio::main]
async fn main() {
    println!("Listening at 0.0.0.0:3030");

    let hello = hello!().with(warp::log("hello"));
    let login = login!().with(warp::log("login"));

    let end = hello.or(login);

    warp::serve(end).run(([0, 0, 0, 0], 3030)).await;
}
