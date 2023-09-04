use rand::Rng;

pub trait Parameter<const N: usize> {
    fn mutate<R: Rng + ?Sized>(&mut self, rng: &mut R) -> &mut Self;

    fn recombine<R: Rng + ?Sized>(rng: &mut R, parents: &[&Self; N]) -> [Self; N]
    where
        Self: Sized;
}

pub struct BoundedVector<T> {
    min_value: T,
    max_value: T,
    value: T,
}
