# ζ Riemann Explorer

[![CI](https://github.com/Guilherme-Bernal/riemann-explorer/actions/workflows/ci.yml/badge.svg)](https://github.com/Guilherme-Bernal/riemann-explorer/actions/workflows/ci.yml)
[![Site](https://img.shields.io/badge/site-no%20ar-b3261e)](https://Guilherme-Bernal.github.io/riemann-explorer/)

[English](README.md) · **Português**

Um caderno interativo sobre a Hipótese de Riemann, os resultados de IA em 2026 (o limite de 67,2% do Claude e a prova da quase-Riemann da OpenAI), os sete Problemas do Milênio e os matemáticos por trás deles. A matemática roda em Rust, compilado para WebAssembly, então o site funciona sem servidor.

**No ar:** https://Guilherme-Bernal.github.io/riemann-explorer/

> A Hipótese de Riemann continua em aberto. Os resultados de 2026 são avanços parciais.

## O que ele calcula

| Seção | Matemática | Rust |
|---|---|---|
| Laboratório Z(t) | Z(t) de Riemann-Siegel, zeros na reta crítica | Riemann-Siegel para localizar, Euler-Maclaurin para refinar (erro < 10⁻¹⁰) |
| Conferência de Riemann-von Mangoldt | N(T) ≈ θ(T)/π + 1 | Compara zeros encontrados com os esperados |
| Paisagem da zeta | log₁₀ \|ζ(s)\| no plano complexo | Somatória de Euler-Maclaurin com termos de Bernoulli |
| Contando primos | π(x), li(x), R(x) e o limite de Schoenfeld | Crivo de Eratóstenes, série de Ramanujan, série de Gram |
| A música dos primos | Fórmula explícita de ψ(x) a partir dos zeros | Soma sobre os zeros da zeta |

## Estrutura

    core/     riemann-core: toda a matemática, sem I/O, com testes
    wasm/     ponte wasm-bindgen usada pelo site
    server/   API axum com o mesmo core, paralela (rayon) e com cache (moka)
    frontend/ Vue 3 + Vite, PT/EN, Rust via WebAssembly (com fallback para a API)

## Rodar localmente

Requisitos: Rust, Node 18+, `wasm-pack`.

    rustup target add wasm32-unknown-unknown
    cargo install wasm-pack
    cd frontend
    npm install
    npm run wasm
    npm run dev

Abra http://localhost:5173

API opcional (http://127.0.0.1:3000):

    cargo run -p riemann-server --release

Rotas: `/api/content?lang=pt|en`, `/api/z`, `/api/zeros`, `/api/heatmap`, `/api/primes`, `/api/psi`.

## Testes

    cargo test -p riemann-core --release

Conferem os 10 primeiros zeros com as tabelas de Odlyzko (erro < 10⁻⁸), ζ(2) = π²/6, ζ(½), a contagem de zeros contra Riemann-von Mangoldt, π(10⁶) = 78.498, li(10⁶), R(10⁶) e a fórmula explícita. O CI roda os testes a cada push e publica o site no GitHub Pages.

## Fontes

- Anthropic, Claude e a Hipótese de Riemann (ago/2026)
- Quanta Magazine, AI Proved the Quasi-Riemann Hypothesis. What Happens Now? (out/2026)
- Clay Mathematics Institute, Millennium Prize Problems
- A. Odlyzko, tabelas de zeros da função zeta de Riemann

Autor: Guilherme Savassa Bernal