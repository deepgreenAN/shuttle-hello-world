use my_crate::hello_world2;

use std::path::Path;

use axum::{
    http::StatusCode,
    routing::{get, get_service},
    Router,
};
use shuttle_runtime::CustomError;
use sqlx::PgPool;
use tower_http::services::ServeDir;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:{secrets.PASSWORD}@localhost/shuttle_example"
    )]
    pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|_| CustomError::msg("migration error."))?;

    let dist_path = Path::new("./public");
    let serve_dir = ServeDir::new(dist_path);

    let router = Router::new().route("/", get(hello_world2)).nest_service(
        "/public",
        get_service(serve_dir).handle_error(|_| async move { StatusCode::INTERNAL_SERVER_ERROR }),
    );

    Ok(router.into())
}
