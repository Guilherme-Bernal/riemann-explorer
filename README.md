# ζ Riemann Explorer

[![CI](https://github.com/Guilherme-Bernal/riemann-explorer/actions/workflows/ci.yml/badge.svg)](https://github.com/Guilherme-Bernal/riemann-explorer/actions/workflows/ci.yml)
[![Site](https://img.shields.io/badge/site-live-b3261e)](https://Guilherme-Bernal.github.io/riemann-explorer/)

**English** · [Português](README.pt-BR.md)

An interactive notebook on the Riemann Hypothesis, the AI results of 2026 (Claude's 67.2% bound and OpenAI's quasi-Riemann proof), the seven Millennium Prize Problems and the mathematicians behind them. The math runs in Rust, compiled to WebAssembly, so the site works with no server.

**Live:** https://Guilherme-Bernal.github.io/riemann-explorer/

> The Riemann Hypothesis is still open. The 2026 results are partial advances.

## What it computes

| Section | Math | Rust |
|---|---|---|
| Z(t) laboratory | Riemann-Siegel Z(t), zeros on the critical line | Riemann-Siegel to locate, Euler-Maclaurin to refine (error < 10⁻¹⁰) |
| Riemann-von Mangoldt check | N(T) ≈ θ(T)/π + 1 | Compares zeros found with zeros expected |
| Zeta landscape | log₁₀ \|ζ(s)\| on the complex plane | Euler-Maclaurin summation with Bernoulli terms |
| Counting primes | π(x), li(x), R(x) and Schoenfeld's bound | Sieve of Eratosthenes, Ramanujan series, Gram series |
| Music of the primes | Explicit formula for ψ(x) from the zeros | Sum over zeta zeros |

## Structure

    core/     riemann-core: all the math, no I/O, with tests
    wasm/     wasm-bindgen bridge used by the site
    server/   axum API using the same core, parallel (rayon) and cached (moka)
    frontend/ Vue 3 + Vite, PT/EN, Rust via WebAssembly (falls back to the API)

## Run locally

Requirements: Rust, Node 18+, `wasm-pack`.

    rustup target add wasm32-unknown-unknown
    cargo install wasm-pack
    cd frontend
    npm install
    npm run wasm
    npm run dev

Open http://localhost:5173

Optional API (http://127.0.0.1:3000):

    cargo run -p riemann-server --release

Routes: `/api/content?lang=pt|en`, `/api/z`, `/api/zeros`, `/api/heatmap`, `/api/primes`, `/api/psi`.

## Tests

    cargo test -p riemann-core --release

They check the first 10 zeros against Odlyzko's tables (error < 10⁻⁸), ζ(2) = π²/6, ζ(½), the zero count against Riemann-von Mangoldt, π(10⁶) = 78,498, li(10⁶), R(10⁶) and the explicit formula. CI runs them on every push and deploys the site to GitHub Pages.

## Sources

- Anthropic, Claude and the Riemann Hypothesis (Aug 2026)
- Quanta Magazine, AI Proved the Quasi-Riemann Hypothesis. What Happens Now? (Oct 2026)
- Clay Mathematics Institute, Millennium Prize Problems
- A. Odlyzko, tables of zeros of the Riemann zeta function

Author: Guilherme Savassa Bernal