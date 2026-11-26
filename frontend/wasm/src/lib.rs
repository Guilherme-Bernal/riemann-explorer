//! Ponte WebAssembly: expõe o riemann-core para o navegador.
//! Todos os retornos são Float64Array achatados.

use riemann_core::{primes, zeta};
use wasm_bindgen::prelude::*;

/// [t0, z0, t1, z1, ...]
#[wasm_bindgen]
pub fn z_curve(from: f64, to: f64, step: f64) -> Vec<f64> {
    zeta::z_curve(from, to.min(2000.0), step)
}

/// Zeros na reta crítica com 10 <= t <= to.
#[wasm_bindgen]
pub fn find_zeros(to: f64) -> Vec<f64> {
    zeta::find_zeros(to.clamp(15.0, 2000.0))
}

/// Número esperado de zeros até T pela fórmula de Riemann-von Mangoldt.
#[wasm_bindgen]
pub fn zero_count_expected(t: f64) -> f64 {
    zeta::zero_count_expected(t)
}

/// log10|zeta(s)| numa grade w x h, sigma em [-0.5, 1.5], t em [0.5, t_max].
#[wasm_bindgen]
pub fn heatmap(t_max: f64, w: usize, h: usize) -> Vec<f64> {
    zeta::heatmap(t_max.clamp(5.0, 200.0), w.clamp(10, 400), h.clamp(10, 800))
}

/// [x, pi(x), li(x), R(x), ...]
#[wasm_bindgen]
pub fn primes_curve(n_max: usize, samples: usize) -> Vec<f64> {
    primes::primes_curve(n_max.min(20_000_000), samples)
}

/// [x, psi(x), psi_aprox(x), ...] usando os k primeiros zeros.
#[wasm_bindgen]
pub fn psi_curve(x_max: f64, samples: usize, zeros: &[f64], k: usize) -> Vec<f64> {
    primes::psi_curve(x_max, samples, zeros, k)
}