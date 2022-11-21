use intmax_zkp_core::{
    rollup::gadgets::deposit_block::DepositInfo,
    sparse_merkle_tree::{
        gadgets::verify::verify_smt::SmtInclusionProof,
        goldilocks_poseidon::WrappedHashOut,
        goldilocks_poseidon::{
            LayeredLayeredPoseidonSparseMerkleTree, NodeDataMemory, PoseidonSparseMerkleTree,
        },
    },
    transaction::block_header::BlockHeader,
    zkdsa::account::Address,
};
use plonky2::{
    field::goldilocks_field::GoldilocksField,
    hash::hash_types::{HashOut, RichField},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BlockInfo<F: RichField> {
    #[serde(bound(
        serialize = "BlockHeader<F>: Serialize",
        deserialize = "BlockHeader<F>: Deserialize<'de>"
    ))]
    pub header: BlockHeader<F>,
    #[serde(bound(
        serialize = "WrappedHashOut<F>: Serialize",
        deserialize = "WrappedHashOut<F>: Deserialize<'de>"
    ))]
    pub transactions: Vec<WrappedHashOut<F>>,
    #[serde(bound(
        serialize = "DepositInfo<F>: Serialize",
        deserialize = "DepositInfo<F>: Deserialize<'de>"
    ))]
    pub deposit_list: Vec<DepositInfo<F>>,
    // diff_tree_proof
    // world_state_tree_proof
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TokenKind<F: RichField> {
    #[serde(bound(
        serialize = "Address<F>: Serialize",
        deserialize = "Address<F>: Deserialize<'de>"
    ))]
    pub contract_address: Address<F>,
    #[serde(bound(
        serialize = "WrappedHashOut<F>: Serialize",
        deserialize = "WrappedHashOut<F>: Deserialize<'de>"
    ))]
    pub variable_index: WrappedHashOut<F>,
}

#[test]
fn test_serde_token_kind() {
    use plonky2::{field::types::Sample, hash::hash_types::HashOut};

    let kind: TokenKind<GoldilocksField> = TokenKind {
        contract_address: Address::rand(),
        variable_index: HashOut::rand().into(),
    };
    let encoded_kind = serde_json::to_string(&kind).unwrap();
    let decoded_kind: TokenKind<GoldilocksField> = serde_json::from_str(&encoded_kind).unwrap();
    assert_eq!(decoded_kind, kind);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Asset<F: RichField> {
    #[serde(bound(
        serialize = "TokenKind<F>: Serialize",
        deserialize = "TokenKind<F>: Deserialize<'de>"
    ))]
    pub kind: TokenKind<F>,
    pub amount: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(bound(
    serialize = "BlockHeader<F>: Serialize, SmtInclusionProof<F>: Serialize, Asset<F>: Serialize",
    deserialize = "BlockHeader<F>: Deserialize<'de>, SmtInclusionProof<F>: Deserialize<'de>, Asset<F>: Deserialize<'de>"
))]
pub struct ReceivedAssetProof<F: RichField> {
    pub is_deposit: bool,
    pub diff_tree_inclusion_proof: (BlockHeader<F>, SmtInclusionProof<F>, SmtInclusionProof<F>),
    pub account_tree_inclusion_proof: SmtInclusionProof<F>,
    pub asset: Asset<F>,
}

#[allow(clippy::type_complexity)]
pub fn make_deposit_proof(
    deposit_list: &[DepositInfo<GoldilocksField>],
    index: Option<usize>,
) -> (
    WrappedHashOut<GoldilocksField>,
    Option<(
        SmtInclusionProof<GoldilocksField>,
        SmtInclusionProof<GoldilocksField>,
    )>,
) {
    let mut inner_deposit_tree =
        LayeredLayeredPoseidonSparseMerkleTree::<NodeDataMemory>::default();
    for leaf in deposit_list {
        inner_deposit_tree
            .set(
                leaf.receiver_address.to_hash_out().into(),
                leaf.contract_address.to_hash_out().into(),
                leaf.variable_index.into(),
                HashOut::from_partial(&[leaf.amount]).into(),
            )
            .unwrap();
    }

    let inner_deposit_root = inner_deposit_tree.get_root();

    let mut deposit_tree = PoseidonSparseMerkleTree::<NodeDataMemory>::default();
    let pseudo_tx_hash = Default::default();
    deposit_tree
        .set(pseudo_tx_hash, inner_deposit_root)
        .unwrap();

    let deposit_root = deposit_tree.get_root();

    if index.is_none() {
        return (deposit_root, None);
    }

    let index = index.unwrap();

    let target_leaf = deposit_list[index];

    let inner_deposit_tree: PoseidonSparseMerkleTree<NodeDataMemory> = inner_deposit_tree.into();
    let deposit_proof2 = inner_deposit_tree
        .find(&target_leaf.receiver_address.to_hash_out().into())
        .unwrap();

    debug_assert!(deposit_proof2.found);

    let deposit_proof1 = deposit_tree.find(&pseudo_tx_hash).unwrap();

    debug_assert!(deposit_proof1.found);

    (deposit_root, Some((deposit_proof1, deposit_proof2)))
}
