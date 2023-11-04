use rand::Rng;

use super::Recombinator;

pub struct OnePoint;

impl Recombinator<i32, 2> for OnePoint {
    fn recombine<R: Rng + ?Sized>(&self, rng: &mut R, parents: &[&i32; 2]) -> [i32; 2] {
        // Use two's complement to get all bits set
        const ALL_BITS: i32 = !0;

        let [&parent_1, &parent_2] = parents;
        let mask = rng.gen_range(1..i32::BITS);

        let child_1 = (parent_1 & (ALL_BITS << mask)) | (parent_2 & !(ALL_BITS << mask));
        let child_2 = (parent_2 & (ALL_BITS << mask)) | (parent_1 & !(ALL_BITS << mask));

        [child_1, child_2]
    }
}
