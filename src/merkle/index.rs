use std::ops::{BitXor, Deref};

use num::One;


fn siblingIndices<N: BitXor<Output = N> + One + Copy>(leaf_indices: &[N]) -> Vec<N> {
    leaf_indices.iter().map(|x| siblingIndex(*x)).collect()
}

///
/// ```rust
/// use merkle_trees::merkle::index::siblingIndex;
/// assert_eq!(100, 0b1100100);
/// assert_eq!(101, 0b1100101);
/// assert_eq!(siblingIndex(100), 0b1100101);
/// assert_eq!(siblingIndex(101), 0b1100100);
/// ```
pub fn siblingIndex<N: BitXor<Output = N> + One>(index: N) -> N {
    index.bitxor(N::one())
}
