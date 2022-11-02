use crate::{prelude::*};

/// Leaves is a representation of slice of leaf 
pub type Leaves<Hash, N> = Vec<Leaf<Hash, N>>;  

/// Layers is representation of slice of Leaves slice
pub type Layers<Hash, N>  = Vec<Leaves<Hash, N>>;


// PartialTree represents a part of the original tree that is enough to calculate the root.
// Used in to extract the root in a merkle proof, to apply diff to a tree or to merge
// multiple trees into one.
// It is a rare case when you need to use this struct on it's own. It's mostly used inside
// Since it's a partial tree, hashes must be accompanied by their index in the original tree.
#[derive(Default, RuntimeDebug, Clone)]
pub struct PartialTree<Hasher, Hash, N> {
    pub layers : Layers<Hash, N>,
    pub _marker : PhantomData<(Hasher, Hash, N)>,
}

// Proof is used to parse, verify, calculate a root for Merkle proofs.
// Proof requires specifying hashing algorithm and hash size in order to work.
// The hashing algorithm is set through the Hasher interface, which is supplied as a generic
// parameter to the Proof.
pub struct  Proof<Hasher, Hash, N> {
    pub proof_hashes : Vec<Vec<u8>>,
    pub leaves: Leaves<Hash, N>,
    pub total_leaves_count: usize,
    pub _marker : PhantomData<Hasher>,
}

impl<Hasher, Hash, N> Proof<Hasher, Hash, N> {
    pub fn new(leaves: Leaves<Hash, N>, proof_hashes: Vec<Vec<u8>>, total_leaves_count: usize) -> Self {
        Self {
            proof_hashes,
            leaves,
            total_leaves_count,
            _marker : <_>::default(),
        }
    }
}