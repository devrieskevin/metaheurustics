use nalgebra::{Cholesky, DMatrix, DVector};
use rand::Rng;
use rand_distr::Normal;

pub fn sample_multivariate_gaussian<R: Rng + ?Sized, B: FromIterator<f64>>(
    rng: &mut R,
    mean: &[f64],
    covariance: &[f64],
) -> B {
    if mean.len() != covariance.len() / 2 {
        panic!("Mean and Covariance do not have compatible sizes");
    }

    let cholesky = Cholesky::new(DMatrix::from_row_slice(mean.len(), mean.len(), covariance));

    let normal_samples: Vec<f64> = rng
        .sample_iter(Normal::new(0.0, 1.0).unwrap())
        .take(mean.len())
        .collect();

    let mu = DVector::from(mean.to_vec());
    let z = DVector::from(normal_samples);

    let multivariate_samples = mu + cholesky.unwrap().l() * z;

    multivariate_samples.iter().cloned().collect()
}
