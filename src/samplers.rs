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
    use nalgebra::{DMatrix, Dyn, Matrix, VecStorage};
    use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};

    use super::sample_multivariate_gaussian;

    const TOLERANCE: f64 = 1e-2;
    const N_SAMPLES: usize = 100_000;

    fn calculate_sample_covariance(
        sample_matrix: Matrix<f64, Dyn, Dyn, VecStorage<f64, Dyn, Dyn>>,
    ) -> Matrix<f64, Dyn, Dyn, VecStorage<f64, Dyn, Dyn>> {
        let sample_mean_vector = sample_matrix.row_mean().transpose();
        let sample_covariance_matrix: Matrix<_, _, _, _> = sample_matrix
            .row_iter()
            .map(|row| &row.transpose() - &sample_mean_vector)
            .map(|x| &x * &x.transpose())
            .sum();

        sample_covariance_matrix / (sample_matrix.nrows() as f64 - 1.0)
    }

    fn test_gaussian_sampler_convergence<R: Rng + ?Sized>(
        rng: &mut R,
        mean: &[f64],
        covariance: &[f64],
        n_samples: usize,
        tolerance: f64,
    ) {
        let samples: Vec<Vec<f64>> =
            sample_multivariate_gaussian(rng, &mean, &covariance, n_samples);
        let sample_matrix =
            DMatrix::from_row_iterator(n_samples, mean.len(), samples.into_iter().flatten());

        let sample_mean = sample_matrix.row_mean();
        let mean_error: f64 = mean
            .iter()
            .zip(&sample_mean)
            .map(|(a, b)| f64::abs(a - b))
            .sum();
        let mean_error = mean_error / mean.len() as f64;

        assert!(
            mean_error < tolerance,
            "Resulting mean error: {} < {}",
            mean_error,
            tolerance
        );

        let sample_covariance_matrix = calculate_sample_covariance(sample_matrix);

        let covariance_matrix = DMatrix::from_row_slice(mean.len(), mean.len(), &covariance);
        let covariance_error = (&sample_covariance_matrix - &covariance_matrix)
            .abs()
            .mean();
        assert!(
            covariance_error < tolerance,
            "Resulting covariance error: {} < {}",
            covariance_error,
            tolerance
        );
    }

    #[test]
    fn test_standard_gaussian_statistics() {
        const SIZE: usize = 3;

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
        let mut rng: StdRng = SeedableRng::seed_from_u64(1234);

        test_gaussian_sampler_convergence(&mut rng, &mean, &covariance, N_SAMPLES, TOLERANCE);
    }

    #[test]
    fn test_nontrivial_gaussian_statistics() {
        let mean = [1.0, 1.0, 1.0];
        let covariance = [2.0, -1.0, 0.0, -1.0, 2.0, -1.0, 0.0, -1.0, 2.0];
        let mut rng: StdRng = SeedableRng::seed_from_u64(1234);

        test_gaussian_sampler_convergence(&mut rng, &mean, &covariance, N_SAMPLES, TOLERANCE);
    }
}
