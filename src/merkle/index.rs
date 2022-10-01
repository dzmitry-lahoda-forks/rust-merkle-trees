use std::ops::{BitAnd, BitXor, Deref, Shl};

use num::{CheckedDiv, One, Zero};

///```rust
/// use merkle_trees::merkle::index::*;
/// let n = vec![0b01, 0b00, 0b111, 0b110];
/// let m = sibling_indices(& n[..]);
/// let p = vec![0b00, 0b01, 0b110, 0b111];
/// assert_eq!(p, m);
///```
pub fn sibling_indices<N: BitXor<Output = N> + One + Copy>(leaf_indices: &[N]) -> Vec<N> {
    leaf_indices.iter().map(|x| sibling_index(*x)).collect()
}

///```rust
/// use merkle_trees::merkle::index::*;
/// let n = vec![0b000,0b001, 0b110, 0b111];
/// let m = parent_indices(& n[..], 2);
/// let p = vec![0b0, 0b11];
/// assert_eq!(p, m);
///```
/// `leaf_indices` - sorted set of indices
pub fn parent_indices<N: BitXor<Output = N> + One + CheckedDiv + Copy + PartialEq>(
    leaf_indices: &[N],
    divider: N,
) -> Vec<N> {
    let mut last = None;
    let mut parents = vec![];
    for index in leaf_indices {
        let parent = parent_index(*index, divider);
        if Some(parent) == last {
            continue;
        }
        parents.push(parent);
        last = Some(parent);
    }
    parents
}

/// ```rust
/// use merkle_trees::merkle::index::*;
/// assert_eq!(is_even(0b100), true);
/// assert_eq!(is_even(0b101), false);
/// ```
pub fn is_even<N: BitAnd<Output = N> + One + Zero + PartialEq>(index: N) -> bool {
    index.bitand(N::one()).eq(&N::zero())
}


/// ```rust
/// use merkle_trees::merkle::index::*;
/// assert_eq!(is_even(0b100), true);
/// assert_eq!(is_even(0b101), false);
/// ```
pub fn get_left_index<N: Shl<Output = N> + One + Zero + PartialEq>(index: N) -> bool {
    index.bitand(N::one()).eq(&N::zero())
}

///
/// ```rust
/// use merkle_trees::merkle::index::*;
/// assert_eq!(100, 0b1100100);
/// assert_eq!(101, 0b1100101);
/// assert_eq!(sibling_index(100), 0b1100101);
/// assert_eq!(sibling_index(101), 0b1100100);
/// ```
pub fn sibling_index<N: BitXor<Output = N> + One>(index: N) -> N {
    index.bitxor(N::one())
}

///
/// ```rust
/// use merkle_trees::merkle::index::*;
/// assert_eq!(parent_index(0b001, 2), 0b0);
/// assert_eq!(parent_index(0b000, 2), 0b0);
/// assert_eq!(parent_index(0b011, 2), 0b1);
/// assert_eq!(parent_index(0b110, 2), 0b11);
/// assert_eq!(parent_index(0b101, 2), 0b10);
/// assert_eq!(parent_index(0b100, 2), 0b10);
/// ```
pub fn parent_index<N: BitXor<Output = N> + One + CheckedDiv>(index: N, divider: N) -> N {
    index.checked_div(&divider).expect("not zero divider")
}
