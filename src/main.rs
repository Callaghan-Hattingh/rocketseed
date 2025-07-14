mod root;
mod transform;

use actix_web::{App, HttpServer};

use crate::root::hello;
use crate::transform::transform_post;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(transform_post))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
