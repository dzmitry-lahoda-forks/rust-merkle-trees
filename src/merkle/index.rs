use std::ops::{Add, BitAnd, BitXor, Deref, Shl};

use num::{CheckedAdd, CheckedDiv, One, Zero};

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
/// let m = parent_indices::<2, _>(& n[..]);
/// let p = vec![0b0, 0b11];
/// assert_eq!(p, m);
///```
/// `leaf_indices` - sorted set of indices
pub fn parent_indices<
    const D: u8,
    N: BitXor<Output = N> + One + CheckedDiv + Copy + PartialEq + From<u8>,
>(
    leaf_indices: &[N],
) -> Vec<N> {
    let mut last = None;
    let mut parents = vec![];
    for index in leaf_indices {
        let parent = parent_index::<D, _>(*index);
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
/// assert_eq!(get_left_index(0b100), 0b1000);
/// assert_eq!(get_left_index(0b101), 0b1010);
/// ```
pub fn get_left_index<N: Shl<Output = N> + One>(index: N) -> N {
    index.shl(N::one())
}

/// ```rust
/// use merkle_trees::merkle::index::*;
/// assert_eq!(get_right_index(0b100), 0b1001);
/// assert_eq!(get_right_index(0b101), 0b1011);
/// ```
pub fn get_right_index<N: Shl<Output = N> + One + Add<Output = N>>(index: N) -> N {
    index.shl(N::one()).add(N::one())
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
/// assert_eq!(parent_index::<2, _>(0b001), 0b0);
/// assert_eq!(parent_index::<2, _>(0b000), 0b0);
/// assert_eq!(parent_index::<2, _>(0b011), 0b1);
/// assert_eq!(parent_index::<2, _>(0b110), 0b11);
/// assert_eq!(parent_index::<2, _>(0b101), 0b10);
/// assert_eq!(parent_index::<2, _>(0b100), 0b10);
/// ```
pub fn parent_index<const D: u8, N: BitXor<Output = N> + One + CheckedDiv + From<u8>>(
    index: N,
) -> N {
    index.checked_div(&D.into()).expect("not zero divider")
}

/// ```rust
/// use merkle_trees::merkle::index::*;
/// let a = vec![0b000,0b001, 0b110, 0b111];
/// let b = vec![0b000, 0b110];
/// let c = extract_new_indices_from_siblings(a.as_ref(), b.as_ref());
/// let d = vec![0b1, 0b111];
/// assert_eq!(c, d);
///```
pub fn extract_new_indices_from_siblings<N: BitXor<Output = N> + One + Copy + PartialEq>(
    sibling_indices: &[N],
    leaf_indices: &[N],
) -> Vec<N> {
    sibling_indices
        .iter()
        .filter_map(|x| {
            if !leaf_indices.contains(x) {
                Some(*x)
            } else {
                None
            }
        })
        .collect()
}
