use actix_web::{get, post, web::BytesMut, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct payload {
    msg: String,
}

#[derive(Deserialize, Serialize)]
struct requestPayload {
    name: String,
}

#[get("/hello")]
pub async fn get_hello() -> Result<HttpResponse, Error> {
    //Json<String> {
    let obj = payload {
        msg: "hello world".to_string(),
    };
    Ok(HttpResponse::Ok().json(obj))
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k
#[post("/say_hello")]
pub async fn say_hello(req: web::Json<requestPayload>) -> Result<HttpResponse, Error> {
    let obj = requestPayload {
        name: req.name.to_string()
    };
    Ok(HttpResponse::Ok().json(format!("hello {}", obj.name)))// obj)) // <- send response
}
