//! riemann-core: a matemática do Riemann Explorer, sem dependência de servidor
//! ou navegador. Usada pelo servidor (axum) e compilada para WebAssembly.

pub mod primes;
pub mod zeta;

#[cfg(test)]
mod tests {
    use super::primes::*;
    use super::zeta::*;

    /// Primeiros zeros não triviais (Odlyzko).
    const KNOWN: [f64; 10] = [
        14.134725141734693,
        21.022039638771555,
        25.010857580145688,
        30.424876125859513,
        32.935061587739189,
        37.586178158825671,
        40.918719012147495,
        43.327073280914999,
        48.005150881167159,
        49.773832477672302,
    ];

    #[test]
    fn first_ten_zeros_match_odlyzko() {
        let z = find_zeros(50.0);
        assert_eq!(z.len(), 10, "esperava 10 zeros até t = 50, achou {}", z.len());
        for (got, want) in z.iter().zip(KNOWN.iter()) {
            assert!((got - want).abs() < 1e-8, "zero {got} difere de {want}");
        }
    }

    #[test]
    fn zeta_at_known_values() {
        use num_complex::Complex64;
        let z2 = zeta(Complex64::new(2.0, 0.0));
        assert!((z2.re - std::f64::consts::PI.powi(2) / 6.0).abs() < 1e-12);
        let zh = zeta(Complex64::new(0.5, 0.0));
        assert!((zh.re + 1.4603545088095868).abs() < 1e-10);
    }

    #[test]
    fn zero_count_matches_riemann_von_mangoldt() {
        for t in [100.0, 300.0, 600.0] {
            let found = find_zeros(t).len() as f64 + 0.0;
            let expected = zero_count_expected(t);
            assert!((found - expected).abs() < 2.5, "T={t}: achou {found}, esperado {expected:.2}");
        }
    }

    #[test]
    fn prime_counting() {
        assert_eq!(prime_pi(100), 25);
        assert_eq!(prime_pi(1_000_000), 78_498);
        assert!((li(1e6) - 78_627.549).abs() < 0.01);
        assert!((riemann_r(1e6) - 78_527.4).abs() < 0.5);
    }

    #[test]
    fn explicit_formula_tracks_psi() {
        let zeros = find_zeros(500.0);
        let curve = psi_curve(60.0, 600, &zeros, zeros.len());
        // Longe dos saltos, a aproximação com ~270 zeros fica perto de psi(x).
        let mut worst: f64 = 0.0;
        for c in curve.chunks(3) {
            let frac = c[0] - c[0].floor();
            if frac > 0.3 && frac < 0.7 {
                worst = worst.max((c[1] - c[2]).abs());
            }
        }
        assert!(worst < 0.6, "pior desvio {worst}");
    }
}