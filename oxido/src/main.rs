#![allow(dead_code)]
mod server;
mod http;
mod server_handler;

use std::env;
use server::Server;
use server_handler::ServerHandler;


fn main() {
    print!("Starting server...\n");
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(ServerHandler::new(public_path));
}
