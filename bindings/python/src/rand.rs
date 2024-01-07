use pyo3::{pyclass, pymethods};
use rand::{rngs::SmallRng, RngCore, SeedableRng};

#[pyclass(name = "SmallRng")]
pub struct PySmallRng {
    rng: SmallRng,
}

#[pymethods]
impl PySmallRng {
    #[new]
    pub fn new(seed: Option<u64>) -> Self {
        match seed {
            Some(seed) => Self::seed_from_u64(seed),
            None => Self::from_entropy(),
        }
    }
}

impl RngCore for PySmallRng {
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.rng.try_fill_bytes(dest)
    }
}

impl SeedableRng for PySmallRng {
    type Seed = <SmallRng as SeedableRng>::Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        Self {
            rng: SmallRng::from_seed(seed),
        }
    }
}
