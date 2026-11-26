use std::f64::consts::PI;

/// Função theta de Riemann-Siegel (expansão assintótica).
pub fn theta(t: f64) -> f64 {
    t / 2.0 * (t / (2.0 * PI)).ln() - t / 2.0 - PI / 8.0 + 1.0 / (48.0 * t) + 7.0 / (5760.0 * t.powi(3))
}

/// Z(t) real: |Z(t)| = |zeta(1/2 + it)|. Zeros de Z = zeros de zeta na reta crítica.
/// Fórmula de Riemann-Siegel com o primeiro termo de correção (válida para t >= 10).
pub fn z(t: f64) -> f64 {
    let a = (t / (2.0 * PI)).sqrt();
    let n = a.floor() as usize;
    let p = a - n as f64;
    let th = theta(t);
    let mut sum = 0.0;
    for k in 1..=n {
        let kf = k as f64;
        sum += (th - t * kf.ln()).cos() / kf.sqrt();
    }
    let c0 = (2.0 * PI * (p * p - p - 1.0 / 16.0)).cos() / (2.0 * PI * p).cos();
    let sign = if (n - 1) % 2 == 0 { 1.0 } else { -1.0 };
    2.0 * sum + sign * (2.0 * PI / t).powf(0.25) * c0
}

/// Procura trocas de sinal de Z(t) e refina por bissecção.
pub fn find_zeros(from: f64, to: f64, h: f64) -> Vec<f64> {
    let mut zeros = Vec::new();
    let mut t0 = from;
    let mut z0 = z(t0);
    while t0 < to {
        let t1 = t0 + h;
        let z1 = z(t1);
        if z0 * z1 < 0.0 {
            let (mut lo, mut hi, mut zlo) = (t0, t1, z0);
            for _ in 0..60 {
                let mid = 0.5 * (lo + hi);
                let zm = z(mid);
                if zlo * zm <= 0.0 { hi = mid; } else { lo = mid; zlo = zm; }
            }
            zeros.push((0.5 * (lo + hi) * 1e6).round() / 1e6);
        }
        t0 = t1;
        z0 = z1;
    }
    zeros
}