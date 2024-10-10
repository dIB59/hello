#[macro_export]
macro_rules! get_db_connection {
    ($pool:expr) => {{
        let conn_async = web::block(move || {
            $pool.clone().get().expect("Failed to get DB connection.")
        })
        .await;
        conn_async.expect("Failed to establish connection")
    }};
}