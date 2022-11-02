use crate::prelude::*;
use super::types::*;



fn reverse_layers<Hash : Clone, N : Clone, const D: u8>(layers: Layers<Hash, N>) -> Layers<Hash, N> {
    let mut layers = layers.clone();
    layers.reverse();
    layers
}

fn pop_layer<Hash : Clone, N : Clone>(layers: Layers<Hash, N>) -> (Leaves<Hash, N>, Layers<Hash, N>) {
    let mut layers = layers.clone();
    let leaves = layers.pop().unwrap();
    (leaves, layers)
}

fn extract_indices_and_hashes<Hash, N>(leaves: Leaves<Hash, N>) -> (Vec<N>, Vec<Vec<Hash>) {
    
}