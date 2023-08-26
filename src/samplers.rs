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
    use rand::thread_rng;

    use super::sample_multivariate_gaussian;

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

    #[test]
    fn test_standard_gaussian_statistics() {
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
        let sample_matrix =
            DMatrix::from_row_iterator(N_SAMPLES, SIZE, samples.into_iter().flatten());

        let sample_mean = sample_matrix.row_mean();
        let mean_error: f64 = mean
            .iter()
            .zip(&sample_mean)
            .map(|(a, b)| f64::abs(a - b))
            .sum();
        let mean_error = mean_error / mean.len() as f64;

        assert!(
            mean_error < TOLERANCE,
            "Resulting mean error: {} < {}",
            mean_error,
            TOLERANCE
        );

        let sample_covariance_matrix = calculate_sample_covariance(sample_matrix);

        let covariance_matrix = DMatrix::from_row_slice(SIZE, SIZE, &covariance);
        let covariance_error = (&sample_covariance_matrix - &covariance_matrix)
            .abs()
            .mean();
        assert!(
            covariance_error < TOLERANCE,
            "Resulting covariance error: {} < {}",
            covariance_error,
            TOLERANCE
        );
    }
}
