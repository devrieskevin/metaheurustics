pub trait Parameter {}

#[derive(Clone)]
pub struct BoundedVector<T>
where
    T: PartialOrd,
{
    pub min_value: T,
    pub max_value: T,
    pub value: Vec<T>,
}

impl<T> Parameter for BoundedVector<T> where T: PartialOrd {}
