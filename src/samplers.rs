use itertools::Itertools;
use nalgebra::{Cholesky, DMatrix, DVector};
use rand::Rng;
use rand_distr::Normal;

pub fn sample_multivariate_gaussian<R: Rng + ?Sized, B: FromIterator<C>, C: FromIterator<f64>>(
    rng: &mut R,
    mean: &[f64],
    covariance: &[f64],
    samples: usize,
) -> B {
    if mean.len() != ((covariance.len() as f64).sqrt().round() as usize) {
        panic!("Mean and Covariance do not have compatible sizes");
    }

    let normal_samples: Vec<Vec<f64>> = rng
        .sample_iter(Normal::new(0.0, 1.0).unwrap())
        .chunks(mean.len())
        .into_iter()
        .take(samples)
        .map(|chunk| chunk.collect_vec())
        .collect();

    let cholesky =
        Cholesky::new(DMatrix::from_row_slice(mean.len(), mean.len(), covariance)).unwrap();
    let mu = DVector::from(mean.to_vec());

    normal_samples
        .into_iter()
        .map(DVector::from)
        .map(|z| &mu + &cholesky.l() * z)
        .map(|multivariate_sample| multivariate_sample.iter().copied().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

    use super::sample_multivariate_gaussian;

    #[test]
    fn test_mean() {
        const SIZE: usize = 10;
        const N_SAMPLES: usize = 10000;
        const TOLERANCE: f64 = 1e-1;

        let mean = [0; SIZE].map(|i| i as f64);
        let covariance: Vec<f64> = [[0; SIZE]; SIZE]
            .iter()
            .enumerate()
            .map(|(i, a)| {
                let mut clone = a.clone();
                clone[i] = 1;
                clone
            })
            .flatten()
            .map(|i| i as f64)
            .collect();
        let mut rng = thread_rng();

        let samples: Vec<Vec<f64>> =
            sample_multivariate_gaussian(&mut rng, &mean, &covariance, N_SAMPLES);
        let sample_mean: Vec<f64> = (0..SIZE)
            .map(|i| {
                let sum: f64 = samples.iter().flatten().skip(i).step_by(SIZE).sum();
                let count = samples.iter().flatten().skip(i).step_by(SIZE).count() as f64;
                sum / count
            })
            .collect();

        let error: f64 = mean
            .iter()
            .zip(sample_mean)
            .map(|(a, b)| f64::abs(a - b))
            .sum();
        let error = error / mean.len() as f64;

        assert!(
            error < TOLERANCE,
            "Resulting mean: {} < {}",
            error,
            TOLERANCE
        );
    }
}
