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

#[derive(Clone)]
pub struct BoundedValue<T>
where
    T: PartialOrd,
{
    pub min_value: T,
    pub max_value: T,
    pub value: T,
}

pub trait GaussianStrategyParameter {}

impl GaussianStrategyParameter for BoundedValue<f64> {}

pub struct SelfAdaptiveGaussianVector<T, S>
where
    T: PartialOrd,
    S: GaussianStrategyParameter,
{
    pub value: BoundedVector<T>,
    pub strategy_parameter: S,
}
