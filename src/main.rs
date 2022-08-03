use routerify::{Router, RouterService};
use hyper::{Body, Response, Server, Request};
use std::{net::SocketAddr, convert::Infallible};

use crate::routes::routes::api_router;
#[path = "./router/index.rs"]
mod routes;


async fn home(_req:Request<Body>) -> Result<Response<Body>,Infallible> {
    let _data="Server is on fire";
    Ok(Response::new(Body::from(_data)))

}
struct State(u64);
#[tokio::main]
async fn main() {

    let port = 8080;
    let router = Router::builder()
    .data(State(100))
    // .middleware(Middleware::pre(logger))
        .scope("/api/v1/",api_router())
    .get("/", home)
    .build().unwrap();
    let service = RouterService::new(router).unwrap();
    let addr = SocketAddr::from(([127,0,0,1],port));
    let server = Server::bind(&addr).serve(service);
    println!("Server in running on port :{}",port);
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
   }

}
