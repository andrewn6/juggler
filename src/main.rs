use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use std::thread;
use std::sync::{Arc, Mutex};
use hyper::body::HttpBody;
use structopt::StructOpt;
use serde::Deserialize;
use hyper::{Client, Request, Body, Method};
use hyper::client::connect::HttpConnector;
use hyper::header::HeaderValue;
use tokio::runtime::Builder;
use tokio::sync::mpsc::{channel, Sender, Receiver};
use log::{info, error};
use env_logger::Env;

#[derive(StructOpt)]
#[structopt(name = "load_balancer")]
struct Opt {
    #[structopt(short = "p", long = "port", default_value = "8080")]
    port: u16,
    #[structopt(short = "s", long = "servers", default_value = "http://localhost:8081,http://localhost:8082")]
    servers: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RequestData {
    message: String,
}

struct LoadBalancer {
    servers: Vec<String>,
    current: Mutex<usize>,
}

impl LoadBalancer {
    fn new(servers: Vec<String>) -> LoadBalancer {
        LoadBalancer {
            servers,
            current: Mutex::new(0),
        }
    }

    fn get_server(&self) -> String {
        let mut current = self.current.lock().unwrap();
        let server = self.servers[*current].clone();
        *current = (*current + 1) % self.servers.len();
        server
    }
}

/* Proxy request */
async fn proxy_request(client: &Client<HttpConnector>, server: &str, request: Request<Body>) -> Result<String, String> {
    let mut proxy_request = request;
    *proxy_request.uri_mut() = server.parse().map_err(|e| format!("Invalid URI: {}", e))?;
    proxy_request.headers_mut().insert("host", HeaderValue::from_str(server).unwrap());
    let response = client.request(proxy_request).await.map_err(|e| format!("Error: {}", e))?;
    let response_body = response.into_body().map_err(|e| format!("Failed to read response body: {:?}", e));
    let response_bytes = hyper::body::to_bytes(response_body).await.map_err(|e| format!("Failed to read response bytes: {:?}", e))?;
    let response_str = String::from_utf8_lossy(&response_bytes).to_string();
    Ok(response_str)
}

async fn handle_request(load_balancer: Arc<LoadBalancer>, client:Client<HttpConnector>, mut stream: TcpStream) -> Result <(), String> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).map_err(|e| format!("Failed to read from stream: {}", e))?;
    let request_str = String::from_utf8_lossy(&buffer[..]).to_string();
    let request = Request::builder()
        .method(Method::POST)
        .uri("/echo")
        .header("content-type", "application/json")
        .body(Body::from(request_str.clone()))
        .map_err(|e| format!("Failed to build request: {}", e))?;
    let server = load_balancer.get_server();
    let response_str = proxy_request(&client, &server, request).await?;
    let response = format!("{} response from server {}: {} \n", request_str, server, response_str);
    stream.write(response.as_bytes()).map_err(|e| format!("Failed to write to stream: {}", e))?;
    Ok(())
}
async fn accept_connection(load_balancer: Arc<LoadBalancer> , client: Client<HttpConnector>, mut receiver: Receiver<TcpStream>) {
    while let Some(stream) = receiver.recv().await {
        let load_balancer = load_balancer.clone();
        let client = client.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_request(load_balancer, client, stream).await {
                error!("{}", e);
            }
        });
    }
}
fn main() {
    println!("placeholder")
}