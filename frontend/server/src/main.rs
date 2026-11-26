//! API do Riemann Explorer: o mesmo riemann-core do WASM, com rayon e cache.

use axum::{extract::Query, routing::get, Json, Router};
use moka::sync::Cache;
use rayon::prelude::*;
use riemann_core::{primes, zeta};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::{Arc, OnceLock};
use tower_http::cors::CorsLayer;

static PT: &str = include_str!("../../frontend/src/i18n/pt.json");
static EN: &str = include_str!("../../frontend/src/i18n/en.json");

fn cache() -> &'static Cache<String, Arc<Value>> {
    static C: OnceLock<Cache<String, Arc<Value>>> = OnceLock::new();
    C.get_or_init(|| Cache::new(256))
}

/// Devolve do cache ou calcula (fora do runtime async, para não travar o tokio).
async fn cached(key: String, f: impl FnOnce() -> Value + Send + 'static) -> Json<Value> {
    if let Some(v) = cache().get(&key) {
        return Json((*v).clone());
    }
    let v = tokio::task::spawn_blocking(f).await.expect("cálculo falhou");
    cache().insert(key, Arc::new(v.clone()));
    Json(v)
}

#[derive(Deserialize)]
struct Lang { lang: Option<String> }

#[derive(Deserialize)]
struct RangeQ { from: Option<f64>, to: Option<f64>, step: Option<f64> }

#[derive(Deserialize)]
struct ToQ { to: Option<f64> }

#[derive(Deserialize)]
struct HeatQ { t_max: Option<f64>, w: Option<usize>, h: Option<usize> }

#[derive(Deserialize)]
struct PrimesQ { max: Option<usize>, samples: Option<usize> }

#[derive(Deserialize)]
struct PsiQ { x: Option<f64>, k: Option<usize>, samples: Option<usize> }

async fn content(Query(q): Query<Lang>) -> Json<Value> {
    let src = if q.lang.as_deref() == Some("en") { EN } else { PT };
    Json(serde_json::from_str(src).expect("i18n inválido"))
}

async fn z_curve(Query(q): Query<RangeQ>) -> Json<Value> {
    let from = q.from.unwrap_or(10.0).max(10.0);
    let to = q.to.unwrap_or(60.0).clamp(from + 1.0, 5000.0);
    let step = q.step.unwrap_or(0.05).max(0.005);
    let key = format!("z:{from}:{to}:{step}");
    cached(key, move || {
        let n = ((to - from) / step) as usize + 1;
        let pts: Vec<[f64; 2]> = (0..n)
            .into_par_iter()
            .map(|i| {
                let t = from + i as f64 * step;
                [t, zeta::z_fast(t)]
            })
            .collect();
        json!({ "points": pts })
    })
    .await
}

/// Zeros em paralelo: blocos de 50 com folga de 0,5 nas bordas, depois filtra cada bloco.
fn zeros_parallel(to: f64) -> Vec<f64> {
    let blocks: Vec<(f64, f64)> = {
        let mut v = Vec::new();
        let mut a = 10.0;
        while a < to {
            v.push((a, (a + 50.0).min(to)));
            a += 50.0;
        }
        v
    };
    let mut zs: Vec<f64> = blocks
        .par_iter()
        .flat_map_iter(|&(a, b)| {
            zeta::find_zeros_range((a - 0.5).max(10.0), b + 0.5)
                .into_iter()
                .filter(move |z| *z > a && *z <= b)
        })
        .collect();
    zs.sort_by(|x, y| x.partial_cmp(y).unwrap());
    zs.dedup_by(|x, y| (*x - *y).abs() < 1e-6);
    zs
}

async fn zeros(Query(q): Query<ToQ>) -> Json<Value> {
    let to = q.to.unwrap_or(100.0).clamp(15.0, 5000.0);
    cached(format!("zeros:{to}"), move || {
        let zs = zeros_parallel(to);
        let expected = zeta::zero_count_expected(to);
        json!({ "count": zs.len(), "expected": expected, "zeros": zs })
    })
    .await
}

async fn heatmap(Query(q): Query<HeatQ>) -> Json<Value> {
    let t_max = q.t_max.unwrap_or(40.0).clamp(5.0, 200.0);
    let w = q.w.unwrap_or(80).clamp(10, 400);
    let h = q.h.unwrap_or(200).clamp(10, 800);
    cached(format!("heat:{t_max}:{w}:{h}"), move || {
        let rows: Vec<Vec<f64>> = (0..h)
            .into_par_iter()
            .map(|j| {
                let t = 0.5 + (t_max - 0.5) * (j as f64 + 0.5) / h as f64;
                zeta::heatmap_row(t, -0.5, 1.5, w)
            })
            .collect();
        json!({ "w": w, "h": h, "t_max": t_max, "sigma": [-0.5, 1.5], "log10_abs": rows.concat() })
    })
    .await
}

async fn primes_route(Query(q): Query<PrimesQ>) -> Json<Value> {
    let max = q.max.unwrap_or(1_000_000).clamp(10, 50_000_000);
    let samples = q.samples.unwrap_or(200).clamp(2, 2000);
    cached(format!("primes:{max}:{samples}"), move || {
        let c = primes::primes_curve(max, samples);
        let pts: Vec<&[f64]> = c.chunks(4).collect();
        json!({ "columns": ["x", "pi", "li", "R"], "points": pts })
    })
    .await
}

async fn psi(Query(q): Query<PsiQ>) -> Json<Value> {
    let x = q.x.unwrap_or(50.0).clamp(3.0, 2000.0);
    let k = q.k.unwrap_or(50).clamp(0, 2000);
    let samples = q.samples.unwrap_or(800).clamp(10, 5000);
    cached(format!("psi:{x}:{k}:{samples}"), move || {
        let mut t_needed = 30.0;
        while zeta::zero_count_expected(t_needed) < k as f64 + 2.0 && t_needed < 2000.0 {
            t_needed += 25.0;
        }
        let zs = zeros_parallel(t_needed);
        let c = primes::psi_curve(x, samples, &zs, k);
        let pts: Vec<&[f64]> = c.chunks(3).collect();
        json!({ "zeros_used": k.min(zs.len()), "columns": ["x", "psi", "psi_explicit"], "points": pts })
    })
    .await
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/content", get(content))
        .route("/api/z", get(z_curve))
        .route("/api/zeros", get(zeros))
        .route("/api/heatmap", get(heatmap))
        .route("/api/primes", get(primes_route))
        .route("/api/psi", get(psi))
        .layer(CorsLayer::permissive());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("API do Riemann Explorer em http://127.0.0.1:3000");
    println!("Rotas: /api/content /api/z /api/zeros /api/heatmap /api/primes /api/psi");
    axum::serve(listener, app).await.unwrap();
}