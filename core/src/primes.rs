//! Contagem de primos, li(x), R(x) de Riemann e a fórmula explícita para psi(x).

use std::f64::consts::PI;

const EULER_GAMMA: f64 = 0.577_215_664_901_532_9;

/// Crivo de Eratóstenes: is_prime[n] para 0..=n_max.
pub fn sieve(n_max: usize) -> Vec<bool> {
    let mut is = vec![true; n_max + 1];
    is[0] = false;
    if n_max >= 1 {
        is[1] = false;
    }
    let mut i = 2;
    while i * i <= n_max {
        if is[i] {
            let mut j = i * i;
            while j <= n_max {
                is[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    is
}

/// Integral logarítmica li(x) pela série de Ramanujan (x > 1).
pub fn li(x: f64) -> f64 {
    let l = x.ln();
    let mut sum = 0.0;
    let mut term = 1.0; // (ln x)^n / (n! 2^{n-1})
    let mut inner = 0.0; // soma de 1/(2k+1) para k = 0..floor((n-1)/2)
    for n in 1..200 {
        let nf = n as f64;
        term *= l / nf;
        if n > 1 {
            term /= 2.0;
        }
        if n % 2 == 1 {
            inner += 1.0 / (nf);
        }
        let sign = if n % 2 == 1 { 1.0 } else { -1.0 };
        let add = sign * term * inner;
        sum += add;
        if n > 2 * (l as usize) + 10 && add.abs() < 1e-17 * sum.abs() {
            break;
        }
    }
    EULER_GAMMA + l.ln() + x.sqrt() * sum
}

/// zeta(s) real para s >= 2 (usado na série de Gram).
fn zeta_real(s: f64) -> f64 {
    let n = 30.0_f64;
    let mut sum = 0.0;
    for k in 1..30 {
        sum += (k as f64).powf(-s);
    }
    sum + n.powf(1.0 - s) / (s - 1.0) + 0.5 * n.powf(-s) + s / 12.0 * n.powf(-s - 1.0)
        - s * (s + 1.0) * (s + 2.0) / 720.0 * n.powf(-s - 3.0)
}

/// R(x) de Riemann pela série de Gram: 1 + sum (ln x)^k / (k k! zeta(k+1)).
pub fn riemann_r(x: f64) -> f64 {
    let l = x.ln();
    let mut sum = 1.0;
    let mut pow_fact = 1.0; // (ln x)^k / k!
    for k in 1..400 {
        let kf = k as f64;
        pow_fact *= l / kf;
        let add = pow_fact / (kf * zeta_real(kf + 1.0));
        sum += add;
        if kf > l && add < 1e-15 * sum {
            break;
        }
    }
    sum
}

/// Curva [x, pi(x), li(x), R(x)] com `samples` pontos em 2..=n_max.
pub fn primes_curve(n_max: usize, samples: usize) -> Vec<f64> {
    let n_max = n_max.clamp(10, 50_000_000);
    let is = sieve(n_max);
    let samples = samples.clamp(2, 2000);
    let mut out = Vec::with_capacity(samples * 4);
    let mut count = 0u64;
    let mut n = 0usize;
    for i in 1..=samples {
        let x = 2 + (n_max - 2) * i / samples;
        while n < x {
            n += 1;
            if is[n] {
                count += 1;
            }
        }
        let xf = x as f64;
        out.extend([xf, count as f64, li(xf), riemann_r(xf)]);
    }
    out
}

/// Total de primos até n.
pub fn prime_pi(n: usize) -> u64 {
    sieve(n).iter().filter(|b| **b).count() as u64
}

/// Função de Chebyshev psi(x): soma de ln p sobre as potências de primos p^k <= x.
/// Retorna [x, psi_real(x), psi_aprox(x)] com a fórmula explícita usando os `k` primeiros zeros.
pub fn psi_curve(x_max: f64, samples: usize, zeros: &[f64], k: usize) -> Vec<f64> {
    let x_max = x_max.clamp(3.0, 2000.0);
    let n_max = x_max.floor() as usize;
    let is = sieve(n_max);
    // Saltos de psi: em cada potência de primo p^m, psi sobe ln p.
    let mut jumps = vec![0.0; n_max + 1];
    for (p, &prime) in is.iter().enumerate().skip(2) {
        if prime {
            let lp = (p as f64).ln();
            let mut q = p;
            while q <= n_max {
                jumps[q] = lp;
                q = match q.checked_mul(p) {
                    Some(v) => v,
                    None => break,
                };
            }
        }
    }
    let gammas = &zeros[..k.min(zeros.len())];
    let samples = samples.clamp(10, 5000);
    let mut out = Vec::with_capacity(samples * 3);
    let mut acc = 0.0;
    let mut n = 1usize;
    for i in 0..samples {
        let x = 1.5 + (x_max - 1.5) * i as f64 / (samples - 1) as f64;
        while n < x.floor() as usize {
            n += 1;
            acc += jumps[n];
        }
        out.extend([x, acc, psi_explicit(x, gammas)]);
    }
    out
}

/// psi_0(x) = x - sum_rho x^rho/rho - ln(2 pi) - 1/2 ln(1 - x^-2), somando pares rho e conjugado.
pub fn psi_explicit(x: f64, gammas: &[f64]) -> f64 {
    let lx = x.ln();
    let sx = x.sqrt();
    let mut s = 0.0;
    for &g in gammas {
        let (sn, cs) = (g * lx).sin_cos();
        s += 2.0 * sx * (0.5 * cs + g * sn) / (0.25 + g * g);
    }
    x - s - (2.0 * PI).ln() - 0.5 * (1.0 - 1.0 / (x * x)).ln()
}