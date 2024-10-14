// let mut conn = pool.get().expect("Failed to get DB connection."); 
// line above blocking code
// let conn_asyc = web::block(move || {
//         pool.clone().get().expect("Failed to get DB connection.")
//         your variable =  anything 
//         your_query(&mut conn, anything)
//     })
//     .await;
// let mut conn = conn_asyc.expect("Failed to establish connection");
// replaces the above code with the following
// let mut conn = get_db_connection_async!(pool);
#[macro_export]
macro_rules! get_db_connection_async {
    ($pool:expr,$query:expr) => {{
        web::block(move || {
            let mut conn = $pool.clone().get().expect("couldn't get db connection from pool");
            $query(&mut conn)
        })
        .await
        .map_err(actix_web::error::ErrorInternalServerError).expect("internal server error")
    }};
}