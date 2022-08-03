

#[path = "./../controller/"]

pub mod routes {
    pub mod register_controller;
    pub use register_controller::register_controller::{register};
    use routerify::Router;
    use hyper::{Body};
    use std::convert::Infallible;

    pub fn api_router() -> Router<Body, Infallible> {

        Router::builder()
            .post("/register", register)
            
            .build()
            .unwrap()
    }

}
