use anyhow::Result;
use hyper::{Body, Request, Response};
use hyper::service::{make_service_fn, service_fn};
use hyper_tls::HttpsConnector;
use std::convert::Infallible;
use tokio::net::TcpListener;
use tokio_rustls::rustls::internal::pemfile::{certs, rsa_private_keys};
use tokio_rustls::rustls::{NoClientAuth, ServerConfig};
use tokio_rustls::TlsAcceptor;

async fn hello(_: Request<Body>) -> Result<Response<Body>> {
    Ok(Response::new(Body::from("Hello World!")))
}
#[tokio::main]
async fn main() {
  let cert_file = std::fs::File::open("cert.pem");
  let key_file = std::fs::File::open("key.pem");

  
}