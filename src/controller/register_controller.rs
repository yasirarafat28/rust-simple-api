
pub mod register_controller {

    use hyper::{Body, Response, Request};
    use std::{convert::Infallible};
    use serde_json;
    use serde_json::Value;
    pub async fn register(_req:Request<Body>)->Result<Response<Body>,Infallible> {
        // let body = (hyper::body::to_bytes(_req.into_body()).await).unwrap().to_vec();
        // // let my_vector: Vec<u8> = request.into_body().data().await.unwrap().unwrap().to_vec();
        // let my_string = String::from_utf8(body).unwrap().to_string();
        // let result: Value = serde_json::from_str(my_string).unwrap();
        // println!("Registering{:?}",result);

        Ok(Response::new(Body::from("List of books")))
    }

}
