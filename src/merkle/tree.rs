use crate::prelude::*;
use std::default::Default;

use super::{types::*, errors::Error};

// Tree is a Merkle Tree that is well suited for both basic and advanced usage.
//
// Basic features include the creation and verification of Merkle proofs from a set of leaves.
// This is often done in various cryptocurrencies.
//
// Advanced features include being able to make transactional changes to a tree with being able to
// roll back to any previously committed state of the tree. This scenario is similar to Git and
// can be found in databases and file systems.
#[derive(Clone, RuntimeDebug, Default)]
pub struct Tree<Hasher, Hash, N> {
    pub current_working_tree: PartialTree<Hasher, Hash, N>,
    pub uncommitted_leaves: Leaves<Hash, N>,
    pub _marker: PhantomData<(Hasher)>,
}
impl<Hasher: Default, Hash: Default, N: Default> Tree<Hasher, Hash, N> {
    /// builds tree from leaves
    pub fn from_leaves(leaves: &mut Leaves<Hash, N>) -> Result<Self, Error> {
        let mut this = Self::default();
        this.uncommitted_leaves.append(leaves);
        // this.commit()?;
        Ok(this)
    }

    // /// commit commits the changes made by insert and append
    // /// and modifies the root.
    // pub fn commit(&mut self) -> Result<(), Error> {
    //     // get difference committed and not committed tree layers
    //     let diff = self.uncommited_diff()?;

    //     Ok(())
    // }

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
