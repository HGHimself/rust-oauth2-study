source .env
diesel database reset --database-url=$DATABASE_URL_TEST
cargo test -- --test-threads=1
