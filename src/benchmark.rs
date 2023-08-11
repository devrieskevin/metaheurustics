/// Implements the bent cigar function.
pub fn bent_cigar(x: &[f64]) -> f64 {
    let sum: f64 = (1..x.len()).map(|i| x[i].powi(2)).sum();
    x[0] * x[0] + 1e6 * sum
}

// Implements the katsuura test objective function.
pub fn katsuura(x: &[f64], d: i32) -> f64 {
    (0..x.len())
        .map(|i: usize| {
            let sum: f64 = (1..=d)
                .map(|k| {
                    let pow2 = 2_f64.powi(k);
                    (pow2 * x[i]).floor() / pow2
                })
                .sum();

            1.0 + (i as f64 + 1.0) * sum
        })
        .product()
}
