# Riemann Explorer

Projeto educativo sobre a Hipótese de Riemann, os avanços com IA em 2026 (o limite de 67,2% do Claude e a quase-Riemann da OpenAI), os 7 Problemas do Milênio e os matemáticos por trás deles.

- backend: Rust + axum. Serve o conteúdo e calcula Z(t) pela fórmula de Riemann-Siegel, encontrando zeros da zeta por bissecção.
- frontend: Vue 3 + Vite.

## Como rodar

Terminal 1:

    cd backend
    cargo run

Terminal 2:

    cd frontend
    npm install
    npm run dev

Depois abra http://localhost:5173

Observação: a Hipótese de Riemann continua aberta. Os resultados de 2026 são avanços parciais.