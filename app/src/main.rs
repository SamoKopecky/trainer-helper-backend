use api::Api;
use clap::{arg, command};
use db::Db;
use migration::{Migrator, MigratorTrait};
use seeder::insert_seeds;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
pub mod crud;
mod db;
mod seeder;
mod settings;

#[tokio::main]
async fn main() {
    let matches = command!()
        .arg(arg!(-s --seed "Seed db").required(false))
        .get_matches();

    let db = Db::build().await.unwrap();
    let _ = Migrator::up(&db.pool, None).await;

    match matches.get_one::<bool>("seed") {
        Some(seed) if *seed == true => insert_seeds().await,
        _ => {}
    }

    let app = Api::build().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:2001").await.unwrap();
    println!("Running API ...");
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .init();

    axum::serve(listener, app).await.unwrap();
}
