use my_crate::hello_world2;

use axum::{routing::get, Router};

// async fn hello_world() -> &'static str {
//     "Hello, world!"
// }

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world2));

    Ok(router.into())
}
