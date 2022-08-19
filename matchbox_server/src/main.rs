use clap::Parser;
use log::{error, info};
use std::env;
use warp::{http::StatusCode, hyper::Method, Filter, Rejection, Reply};

pub use args::Args;
pub use signaling::matchbox::PeerId;

mod args;
mod signaling;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "matchbox_server=info");
    }
    pretty_env_logger::init();
    let args = Args::parse();

    let health_route = warp::path("health").and_then(health_handler);

    let log = warp::log("made_in_heaven");

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "Access-Control-Allow-Headers",
            "Access-Control-Request-Method",
            "Access-Control-Request-Headers",
            "Origin",
            "Accept",
            "X-Requested-With",
            "Content-Type",
        ])
        .allow_methods(&[
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
            Method::HEAD,
        ]);

    let routes = health_route
        .or(signaling::ws_filter(Default::default()))
        .with(cors)
        .with(log);

    if args.tls {
        // Ensure the cert and key are present on disk and accessible
        let cert_path = std::path::Path::new(&args.cert_path).try_exists();
        let key_path = std::path::Path::new(&args.key_path).try_exists();
        let mut path_error = false;
        if let Err(e) = cert_path {
            error!("Error accessing certificate: {}", e);
            path_error = true;
        } else if let Ok(false) = cert_path {
            error!("Certificate not found: {}", args.cert_path);
            path_error = true;
        }

        if let Err(e) = key_path {
            error!("Error accessing key file: {}", e);
            path_error = true;
        } else if let Ok(false) = key_path {
            error!("Key file not found: {}", args.key_path);
            path_error = true;
        }

        if path_error {
            return;
        }

        info!(
            "Starting matchbox signaling server with TLS at port {}",
            args.host.port()
        );
        warp::serve(routes)
            .tls()
            .cert_path(args.cert_path)
            .key_path(args.key_path)
            .run(args.host)
            .await;
    } else {
        info!(
            "Starting matchbox signaling server at port {}",
            args.host.port()
        );
        warp::serve(routes).run(args.host).await;
    }
}

pub async fn health_handler() -> std::result::Result<impl Reply, Rejection> {
    Ok(StatusCode::OK)
}
