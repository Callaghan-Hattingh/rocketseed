use actix_web::{post, web, HttpResponse, Responder};
use kuchikiki::{parse_html, traits::TendrilSink};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum TransformType {
    #[serde(rename = "uppercase")]
    Uppercase,
    #[serde(rename = "lowercase")]
    Lowercase,
}

#[derive(Debug, Deserialize, Serialize)]
struct TransformRequest {
    transform: TransformType,
    html: String,
}

impl TransformRequest {
    fn validate(&self) -> Result<(), TransformError> {
        if self.html.trim().is_empty() {
            return Err(TransformError::ParseError(
                "HTML input is empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Error, Debug)]
enum TransformError {
    #[error("Failed to parse HTML: {0}")]
    ParseError(String),
    #[error("Body tag not found")]
    BodyNotFound,
}

fn set_case_text_nodes(node: &kuchikiki::NodeRef, case: &TransformType) {
    for child in node.children() {
        if let Some(text_ref) = child.as_text() {
            // println!("p: {}", text_ref.borrow());
            if case == &TransformType::Lowercase {
                let lowercase_text = text_ref.borrow().to_lowercase();
                *text_ref.borrow_mut() = lowercase_text;
            } else if case == &TransformType::Uppercase {
                let uppercase_text = text_ref.borrow().to_uppercase();
                *text_ref.borrow_mut() = uppercase_text;
            }
        } else {
            set_case_text_nodes(&child, case);
        }
    }
}

fn update_p_elements(html: &str, case: &TransformType) -> Result<String, TransformError> {
    // println!("html: {}", html);
    let mut result_html = Vec::new();
    let document = parse_html().one(html);

    let p_elements = match document.select("p") {
        Ok(elements) => elements,
        Err(_) => return Err(TransformError::ParseError("".to_string())),
    };

    let body = match document.select_first("body") {
        Ok(binding) => binding,
        Err(_) => return Err(TransformError::BodyNotFound),
    };

    for p_element in p_elements {
        let as_node = p_element.as_node();
        set_case_text_nodes(as_node, case);
    }

    for child in body.as_node().children() {
        // println!("{}", child.to_string());
        result_html.push(child.to_string());
    }

    return Ok(result_html.join(""));
}

#[post("/transform")]
async fn transform_post(
    req: Result<web::Json<TransformRequest>, actix_web::Error>,
) -> impl Responder {
    match req {
        Ok(json) => {
            if let Err(e) = json.validate() {
                // log validation error
                return HttpResponse::BadRequest().body(format!("Invalid request: {}", e));
            }

            match update_p_elements(&json.html, &json.transform) {
                Ok(result) => HttpResponse::Ok().body(format!("{}", result)),
                // log processing error
                Err(e) => HttpResponse::BadRequest().body(format!("Invalid html: {}", e)),
            }
        }
        // log request error
        Err(e) => HttpResponse::BadRequest().body(format!("Invalid request: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_transform_endpoint_1() {
        let app = test::init_service(App::new().service(transform_post)).await;

        let payload = TransformRequest {
            transform: TransformType::Uppercase,
            html: "<p>Hello world</p>".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/transform")
            .insert_header(("Content-Type", "application/json"))
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        // println!("Response status: {}", resp.status());

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).expect("Failed to parse body");
        // println!("Response body: {}", body_str);
        assert_eq!(body_str, "<p>HELLO WORLD</p>");
    }

    #[actix_web::test]
    async fn test_transform_endpoint_2() {
        let app = test::init_service(App::new().service(transform_post)).await;

        let payload = TransformRequest {
            transform: TransformType::Lowercase,
            html: "<p>Hello WORLD</p>".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/transform")
            .insert_header(("Content-Type", "application/json"))
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        // println!("Response status: {}", resp.status());

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).expect("Failed to parse body");
        // println!("Response body: {}", body_str);
        assert_eq!(body_str, "<p>hello world</p>");
    }

    #[actix_web::test]
    async fn test_transform_endpoint_3() {
        let app = test::init_service(App::new().service(transform_post)).await;

        let payload = TransformRequest {
            transform: TransformType::Uppercase,
            html: "<div><p>First paragraph</p><span>Not a paragraph</span><p>Second paragraph</p></div>".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/transform")
            .insert_header(("Content-Type", "application/json"))
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        // println!("Response status: {}", resp.status());

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).expect("Failed to parse body");
        // println!("Response body: {}", body_str);
        assert_eq!(
            body_str,
            "<div><p>FIRST PARAGRAPH</p><span>Not a paragraph</span><p>SECOND PARAGRAPH</p></div>"
        );
    }

    #[actix_web::test]
    async fn test_transform_endpoint_4() {
        let app = test::init_service(App::new().service(transform_post)).await;

        let payload = TransformRequest {
            transform: TransformType::Uppercase,
            html: "<p>Text with <strong>bold</strong> and <em>italic</em> elements</p>".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/transform")
            .insert_header(("Content-Type", "application/json"))
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), actix_web::http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).expect("Failed to parse body");
        // println!("Response body: {}", body_str);
        assert_eq!(
            body_str,
            "<p>TEXT WITH <strong>BOLD</strong> AND <em>ITALIC</em> ELEMENTS</p>"
        );
    }
}
