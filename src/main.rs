use std::{
    net::{Ipv6Addr, SocketAddr, SocketAddrV6},
    path::PathBuf,
};

use axum::{http::Request, routing::get, Json, Router};
use clap::Parser;
use serde_json::{json, Value};

mod config;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short = 'c', long, default_value = "config.yaml", env)]
    config: PathBuf,
    #[clap(short = 'l', long, env)]
    listen: Option<SocketAddr>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = config::Config::parse(args.config).await?;
    log4rs::init_raw_config(config.logging)?;

    let sockaddr = args.listen.unwrap_or_else(|| {
        config
            .server
            .listen
            .unwrap_or(std::net::SocketAddr::V6(SocketAddrV6::new(
                Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
                8080,
                0,
                0,
            )))
    });

    let listener = tokio::net::TcpListener::bind(sockaddr).await?;

    let app = Router::new().route("/hello", get(hello));

    log::info!("Listening on {sockaddr:?}");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

async fn hello<B>(req: Request<B>) -> Json<Value> {
    let connection_info = req
        .extensions()
        .get::<axum::extract::connect_info::ConnectInfo<SocketAddr>>()
        .unwrap();
    let ip_address = connection_info.0.ip();

    log::debug!("Request from {ip_address:?}");

    Json(json!({"hello": "world"}))
}
