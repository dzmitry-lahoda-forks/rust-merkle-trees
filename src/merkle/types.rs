use crate::{prelude::*};

/// Leaves is a representation of slice of leaf 
pub type Leaves<Hash, N> = Vec<Leaf<Hash, N>>;  

/// Layers is representation of slice of Leaves slice
pub type Layers<Hash, N>  = Vec<Leaves<Hash, N>>;


// Tree is a Merkle Tree that is well suited for both basic and advanced usage.
//
// Basic features include the creation and verification of Merkle proofs from a set of leaves.
// This is often done in various cryptocurrencies.
//
// Advanced features include being able to make transactional changes to a tree with being able to
// roll back to any previously committed state of the tree. This scenario is similar to Git and
// can be found in databases and file systems.
#[derive(Default)]
pub struct Tree<Hasher, Hash, N> {
    pub current_working_tree : PartialTree<Hasher, Hash, N>,
    pub uncommitted_leaves : Leaves<Hash, N>,
    pub _marker : PhantomData<(Hasher)>,
}

impl<Hasher, Hash, N> Tree<Hasher, Hash, N> {    
//     /// builds tree from leaves
//     pub fn from_leaves(leaves: Leaves) -> Result<Self, Error> {
//         let mut this = Self::new();
//         this.uncommitted_leaves.append(&mut leaves)?;
//         this.commit()?;
//         Ok(tree)
//     }

//     /// commit commits the changes made by insert and append
//     /// and modifies the root.
//     pub fn commit(&mut self) -> Result<(), Error> {
//         // get difference committed and not committed tree layers
//         let diff = self.uncommited_diff()?;
        
//         Ok(())
//     }

//   /// uncommittedDiff creates a diff from a changes that weren't committed to the main tree yet. Can be used
// /// to get uncommitted root or can be merged with the main tree
//     pub fn uncommited_diff(&self) -> Result<PartialTree<Hasher>, Error> {
//         if self.uncommitted_leaves.len() == 0 {
//             Ok(<_>::default())
//         }
//         else {
//             let (partial_tree_layers, uncommitted_tree_depth) = self.uncommited_partial_tree_layers();
            
//         }
//     }
 }

// PartialTree represents a part of the original tree that is enough to calculate the root.
// Used in to extract the root in a merkle proof, to apply diff to a tree or to merge
// multiple trees into one.
// It is a rare case when you need to use this struct on it's own. It's mostly used inside
// Since it's a partial tree, hashes must be accompanied by their index in the original tree.
#[derive(Default, RuntimeDebug)]
pub struct PartialTree<Hasher, Hash, N> {
    pub layers : Layers<Hash, N>,
    pub _marker : PhantomData<(Hasher, Hash, N)>,
}

// // Proof is used to parse, verify, calculate a root for Merkle proofs.
// // Proof requires specifying hashing algorithm and hash size in order to work.
// // The hashing algorithm is set through the Hasher interface, which is supplied as a generic
// // parameter to the Proof.
// pub struct  Proof<Hasher> {
//     pub proof_hashes : Vec<Vec<u8>>,
//     pub leaves: Leaves,
//     pub total_leaves_count: usize,
//     pub _marker : PhantomData<Hasher>,
// }

// impl<Hasher> Proof<Hasher> {
//     pub fn new(leaves: Leaves, proof_hashes: Vec<Vec<u8>>, total_leaves_count: usize) -> Self {
//         Self {
//             proof_hashes,
//             leaves,
//             total_leaves_count,
//             _marker : <_>::default(),
//         }
//     }
// }pub mod types;