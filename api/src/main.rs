mod graph;

#[macro_use]
extern crate juniper;

use crate::graph::*;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use juniper_actix::{graphql_handler, playground_handler};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = move || {
        let schema = create_schema();

        App::new()
            .data(schema)
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
            .service(web::resource("/playground").route(web::get().to(playground_route)))
            .service(web::resource("/health_check").route(web::get().to(health_check_route)))
    };

    let port = 8080;
    let address = env::var("ADDRESS").unwrap_or("0.0.0.0".to_string());
    let bind = format!("{}:{}", address, port);
    println!("running server on {}", bind);
    HttpServer::new(app).bind(bind)?.run().await?;

    Ok(())
}

async fn health_check_route() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("ok"))
}

async fn playground_route() -> actix_web::Result<HttpResponse> {
    playground_handler("/graphql", None).await
}

async fn graphql_route(
    req: HttpRequest,
    payload: web::Payload,
    schema: web::Data<Schema>,
) -> actix_web::Result<HttpResponse> {
    let authenticated_user_id: Option<String> = match req.headers().get("x-user-id") {
        Some(v) => v.to_str().map(|id| Some(id.to_string())).unwrap_or(None),
        None => authenticate(&req).await.into(),
    };

    if let Some(id) = authenticated_user_id.clone() {
        println!("login user id: {}", id);
    }

    let context = Context::new(authenticated_user_id);
    graphql_handler(&schema, &context, req, payload).await
}

async fn authenticate(req: &HttpRequest) -> Option<String> {
    let token_header: Option<String> = match req.headers().get("authorization") {
        Some(v) => v.to_str().map(|id| Some(id.to_string())).unwrap_or(None),
        None => None,
    };

    if let None = token_header {
        return None;
    }

    let token = token_header.unwrap_or("".to_string());
    if token.len() < 7 {
        return None;
    }

    let result = app::firebase::auth::verify_id_token(&token[7..]).await;

    match result {
        Ok(id) => Some(id),
        Err(_) => None,
    }
}
