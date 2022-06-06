/// `Leaf` is the type of leaf used in the merkle tree and mmr 
pub struct Leaf {
    pub index : u64,
    pub hash : Vec<u8>,
}