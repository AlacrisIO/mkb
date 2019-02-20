use merkle_cbt;
use merkle_cbt::Merge;
use merkle_cbt::MerkleTree;
use numext_fixed_hash::H256;

use types::*;

//#[derive(Default)]
pub struct AllDataMerkleTree {
    pub account_map : MerkleTree<H256>,
    pub token_map : MerkleTree<H256>,
    pub transaction_map : MerkleTree<H256>,
}



// This function takes the request, check for correctness.
// If correct, the signature is returned to be checked.
// If not correct, then the signature is sent in order to be checked.
pub fn get_signature(_eval: SumTypeRequest) -> MerkleVerification {
    let merkl = MerkleVerification { result: true, signature: None };
    merkl
}
