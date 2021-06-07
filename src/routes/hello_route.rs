use warp::{filters::BoxedFilter, Filter};

// 1. "hello"
fn path_prefix() -> BoxedFilter<()> {
    warp::path("hello").boxed()
}

// 2. / String
pub fn hello_prime() -> BoxedFilter<(String,)> {
    warp::get() // 3.
        .and(path_prefix()) // 4.
        .and(warp::path::param::<String>()) // 5.
        .boxed()
}

pub fn hello() -> BoxedFilter<(u64,)> {
    warp::get() // 3.
        .and(path_prefix()) // 4.
        .and(warp::path::param::<u64>()) // 5.
        .boxed()
}
