use crate::prelude::*;
use super::types::*;



fn reverse_layers<Hash : Clone, N : Clone, const D: u8>(layers: Layers<Hash, N>) -> Layers<Hash, N> {
    let mut layers = layers.clone();
    layers.reverse();
    layers
}
