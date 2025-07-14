use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_hello_endpoint() {
        let app = test::init_service(App::new().service(hello)).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello, World!");
    }
}
