use crate::prelude::*;

/// `Leaf` is the type of leaf used in the merkle tree and mmr 
#[derive(RuntimeDebug, Clone)]
pub struct Leaf<Hash, N> {
    pub index : N,
    pub hash : Hash,
}