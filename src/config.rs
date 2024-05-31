use std::{
    net::{SocketAddr, ToSocketAddrs},
    path::Path,
};

use serde::{Deserialize, Deserializer};
use tokio::fs::read;

#[derive(serde::Deserialize, Debug)]
pub struct Server {
    #[serde(deserialize_with = "flatten_resolve_addr")]
    pub listen: Option<SocketAddr>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub server: Server,
    pub logging: log4rs::config::RawConfig,
}

impl Config {
    pub async fn parse(path: impl AsRef<Path>) -> anyhow::Result<Config> {
        Ok(serde_yaml::from_slice(&read(path).await?)?)
    }
}

fn flatten_resolve_addr<'de, D>(de: D) -> Result<Option<SocketAddr>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    Ok(s.to_socket_addrs()
        .map_err(serde::de::Error::custom)?
        .next())
}
