#[macro_export]
macro_rules! login {
    () => {
        login_route::login()
            .and_then(login_handler::login)
            .or(login_route::accept_login().and_then(login_handler::accept_login))
    };
}
