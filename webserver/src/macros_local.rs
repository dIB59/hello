// let mut conn = pool.get().expect("Failed to get DB connection."); 
// line above blocking code
// let conn_asyc = web::block(move || {
//         pool.clone().get().expect("Failed to get DB connection.")
//     })
//     .await;
// let mut conn = conn_asyc.expect("Failed to establish connection");
// replaces the above code with the following
// let mut conn = get_db_connection!(pool);
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