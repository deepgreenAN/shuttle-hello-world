use my_crate::hello_world2;

use axum::{routing::get, Router};
use shuttle_runtime::CustomError;
use sqlx::PgPool;

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

    let router = Router::new().route("/", get(hello_world2));

    Ok(router.into())
}
