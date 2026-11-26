use axum::{extract::Query, routing::get, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_http::cors::CorsLayer;

mod zeta;

static CONTENT: &str = include_str!("../content.json");

#[derive(Deserialize)]
struct RangeQ { from: Option<f64>, to: Option<f64>, step: Option<f64> }

#[derive(Deserialize)]
struct ZerosQ { to: Option<f64> }

async fn content() -> Json<Value> {
    Json(serde_json::from_str(CONTENT).expect("content.json inválido"))
}

async fn z_curve(Query(q): Query<RangeQ>) -> Json<Value> {
    let from = q.from.unwrap_or(10.0).max(10.0);
    let to = q.to.unwrap_or(60.0).min(500.0).max(from + 1.0);
    let step = q.step.unwrap_or(0.05).max(0.005);
    let mut pts = Vec::new();
    let mut t = from;
    while t <= to {
        pts.push([t, zeta::z(t)]);
        t += step;
    }
    Json(json!({ "points": pts }))
}

async fn zeros(Query(q): Query<ZerosQ>) -> Json<Value> {
    let to = q.to.unwrap_or(100.0).clamp(15.0, 1000.0);
    let zs = zeta::find_zeros(10.0, to, 0.01);
    Json(json!({ "count": zs.len(), "zeros": zs }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/content", get(content))
        .route("/api/z", get(z_curve))
        .route("/api/zeros", get(zeros))
        .layer(CorsLayer::permissive());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("API rodando em http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}