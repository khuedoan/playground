use axum::{
    extract::Query,
    routing::{get, post},
    Router,
};
use askama::Template;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/api/age", post(age));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct RootParams {
    name: String
}

#[derive(Template)]
#[template(path = "index.html")]
struct RootTemplate {
    name: String,
}

#[derive(Template)]
#[template(path = "age.html")]
struct AgeTemplate {
    age: i32,
}

async fn root(Query(params): Query<RootParams>) -> RootTemplate {
    RootTemplate {
        name: params.name,
    }
}

async fn age(Query(params): Query<RootParams>) -> AgeTemplate {
    AgeTemplate {
        age: 123,
    }
}
