//! Função zeta de Riemann: Riemann-Siegel (rápido) e Euler-Maclaurin (preciso).

use num_complex::Complex64;
use std::f64::consts::PI;

/// Números de Bernoulli B2, B4, ..., B20.
const BERNOULLI: [f64; 10] = [
    1.0 / 6.0,
    -1.0 / 30.0,
    1.0 / 42.0,
    -1.0 / 30.0,
    5.0 / 66.0,
    -691.0 / 2730.0,
    7.0 / 6.0,
    -3617.0 / 510.0,
    43867.0 / 798.0,
    -174611.0 / 330.0,
];

/// Função theta de Riemann-Siegel (expansão assintótica, ótima para t >= 10).
pub fn theta(t: f64) -> f64 {
    t / 2.0 * (t / (2.0 * PI)).ln() - t / 2.0 - PI / 8.0
        + 1.0 / (48.0 * t)
        + 7.0 / (5760.0 * t.powi(3))
        + 31.0 / (80640.0 * t.powi(5))
}

/// Z(t) pela fórmula de Riemann-Siegel com o termo de correção C0.
/// Custo O(sqrt(t)): usado para desenhar a curva e localizar trocas de sinal.
pub fn z_fast(t: f64) -> f64 {
    let a = (t / (2.0 * PI)).sqrt();
    let n = a.floor().max(1.0) as usize;
    let p = a - n as f64;
    let th = theta(t);
    let mut sum = 0.0;
    for k in 1..=n {
        let kf = k as f64;
        sum += (th - t * kf.ln()).cos() / kf.sqrt();
    }
    let c0 = (2.0 * PI * (p * p - p - 1.0 / 16.0)).cos() / (2.0 * PI * p).cos();
    let sign = if n % 2 == 1 { 1.0 } else { -1.0 };
    2.0 * sum + sign * (2.0 * PI / t).powf(0.25) * c0
}

/// zeta(s) por somatório de Euler-Maclaurin. Preciso em toda a faixa crítica
/// (e fora dela), exceto no polo s = 1. Custo O(|t|).
pub fn zeta(s: Complex64) -> Complex64 {
    let one = Complex64::new(1.0, 0.0);
    let n = (s.im.abs() as usize) + 25;
    let nf = n as f64;
    let mut sum = Complex64::new(0.0, 0.0);
    for k in 1..n {
        sum += (-s * (k as f64).ln()).exp();
    }
    let n_pow = (-s * nf.ln()).exp(); // N^{-s}
    sum += n_pow * nf / (s - one); // N^{1-s}/(s-1)
    sum += n_pow * 0.5;
    // Termos de Bernoulli: B_{2k}/(2k)! * s(s+1)...(s+2k-2) * N^{-s-2k+1}
    let mut poch = s; // s(s+1)...(s+2k-2)
    let mut fact = 2.0; // (2k)!
    let mut npow = n_pow / nf; // N^{-s-1}
    for (i, b) in BERNOULLI.iter().enumerate() {
        let k = (i + 1) as f64;
        sum += poch * (*b / fact) * npow;
        poch = poch * (s + 2.0 * k - 1.0) * (s + 2.0 * k);
        fact *= (2.0 * k + 1.0) * (2.0 * k + 2.0);
        npow /= nf * nf;
    }
    sum
}

/// Z(t) exato (a menos do erro de ponto flutuante): Re(e^{i theta} zeta(1/2 + it)).
pub fn z_precise(t: f64) -> f64 {
    let z = zeta(Complex64::new(0.5, t));
    (Complex64::from_polar(1.0, theta(t)) * z).re
}

/// Curva de Z(t) amostrada, achatada como [t0, z0, t1, z1, ...].
pub fn z_curve(from: f64, to: f64, step: f64) -> Vec<f64> {
    let from = from.max(10.0);
    let step = step.max(0.002);
    let count = ((to - from) / step).floor().max(0.0) as usize + 1;
    let mut out = Vec::with_capacity(count * 2);
    for i in 0..count {
        let t = from + i as f64 * step;
        out.push(t);
        out.push(z_fast(t));
    }
    out
}

/// Refina um zero com o método da secante usando Z preciso.
fn refine(mut a: f64, mut b: f64) -> f64 {
    let mut fa = z_precise(a);
    let mut fb = z_precise(b);
    for _ in 0..30 {
        if fa * fb > 0.0 {
            break;
        }
        let m = b - fb * (b - a) / (fb - fa);
        let fm = z_precise(m);
        if fm == 0.0 || (b - a).abs() < 1e-12 {
            return m;
        }
        // Illinois: mantém o intervalo que contém a raiz.
        if fa * fm < 0.0 {
            b = m;
            fb = fm;
            fa *= 0.5;
        } else {
            a = m;
            fa = fm;
            fb *= 0.5;
        }
    }
    0.5 * (a + b)
}

/// Encontra os zeros de zeta na reta crítica com 10 <= t <= to.
/// Localiza com Riemann-Siegel e refina com Euler-Maclaurin (erro < 1e-8).
pub fn find_zeros(to: f64) -> Vec<f64> {
    find_zeros_range(10.0, to)
}

/// Zeros num intervalo [from, to] (from >= 10). Para paralelizar, use intervalos com folga e filtre.
pub fn find_zeros_range(from: f64, to: f64) -> Vec<f64> {
    let h = 0.01;
    let mut zeros = Vec::new();
    let mut t0 = from.max(10.0);
    let mut z0 = z_fast(t0);
    while t0 < to {
        let t1 = t0 + h;
        let z1 = z_fast(t1);
        if z0 * z1 < 0.0 {
            // Bissecção rápida até 1e-4 e depois secante com Z preciso.
            let (mut lo, mut hi, mut zlo) = (t0, t1, z0);
            while hi - lo > 1e-4 {
                let mid = 0.5 * (lo + hi);
                let zm = z_fast(mid);
                if zlo * zm <= 0.0 {
                    hi = mid;
                } else {
                    lo = mid;
                    zlo = zm;
                }
            }
            // Riemann-Siegel com só C0 erra alguns milésimos para t pequeno:
            // abre um intervalo em volta até Z preciso trocar de sinal.
            let mid = 0.5 * (lo + hi);
            let mut root = mid;
            for w in [0.005, 0.02, 0.05] {
                if z_precise(mid - w) * z_precise(mid + w) < 0.0 {
                    root = refine(mid - w, mid + w);
                    break;
                }
            }
            if root <= to {
                zeros.push(root);
            }
        }
        t0 = t1;
        z0 = z1;
    }
    zeros
}

/// Número esperado de zeros com 0 < Im(rho) <= T (Riemann-von Mangoldt, sem S(T)).
pub fn zero_count_expected(t: f64) -> f64 {
    theta(t) / PI + 1.0
}

/// Mapa de log10|zeta(s)| em sigma em [s0, s1] e t em [t0, t1], linha a linha (t crescente).
pub fn heatmap_row(t: f64, s0: f64, s1: f64, w: usize) -> Vec<f64> {
    (0..w)
        .map(|i| {
            let sigma = s0 + (s1 - s0) * (i as f64 + 0.5) / w as f64;
            let z = zeta(Complex64::new(sigma, t));
            z.norm().max(1e-12).log10()
        })
        .collect()
}

pub fn heatmap(t_max: f64, w: usize, h: usize) -> Vec<f64> {
    let mut out = Vec::with_capacity(w * h);
    for j in 0..h {
        let t = 0.5 + (t_max - 0.5) * (j as f64 + 0.5) / h as f64;
        out.extend(heatmap_row(t, -0.5, 1.5, w));
    }
    out
}