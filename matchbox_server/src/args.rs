use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[clap(
    name = "made_in_heaven",
    rename_all = "kebab-case",
    rename_all_env = "screaming-snake"
)]
pub struct Args {
    #[clap(default_value = "0.0.0.0:3536", env)]
    pub host: SocketAddr,
    #[clap(long, env)]
    pub tls: bool,
    #[clap(default_value = "./cert.crt", env)]
    pub cert_path: String,
    #[clap(default_value = "./cert.key", env)]
    pub key_path: String,
}
