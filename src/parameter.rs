use rand::Rng;

pub trait Parameter<const N: usize> {
    fn mutate<R: Rng + ?Sized>(&mut self, rng: &mut R) -> &mut Self;

    fn recombine<R: Rng + ?Sized>(rng: &mut R, parents: &[&Self; N]) -> [Self; N]
    where
        Self: Sized;
}

pub struct BoundedVector<T>
where
    T: PartialOrd,
{
    pub min_value: T,
    pub max_value: T,
    pub value: Vec<T>,
}
