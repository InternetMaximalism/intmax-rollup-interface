use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};
use serde::{Deserialize, Serialize};

use intmax_zkp_core::{
    merkle_tree::tree::MerkleProof,
    rollup::{block::BlockInfo, gadgets::deposit_block::DepositInfo},
    sparse_merkle_tree::{
        gadgets::{process::process_smt::SmtProcessProof, verify::verify_smt::SmtInclusionProof},
        goldilocks_poseidon::{GoldilocksHashOut, WrappedHashOut, Wrapper},
        proof::{SparseMerkleInclusionProof, SparseMerkleProcessProof},
    },
    transaction::{
        asset::{Asset, ReceivedAssetProof},
        block_header::BlockHeader,
        circuits::MergeAndPurgeTransitionProofWithPublicInputs,
        gadgets::merge::MergeProof,
    },
    zkdsa::{
        account::{Address, PublicKey},
        circuits::SimpleSignatureProofWithPublicInputs,
    },
};

type K = GoldilocksHashOut;
type V = GoldilocksHashOut;
type I = GoldilocksHashOut;

type C = PoseidonGoldilocksConfig;
// type H = <C as GenericConfig<D>>::InnerHasher;
type F = <C as GenericConfig<D>>::F;
const D: usize = 2;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseSingleMessage {
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseResetStateBody {
    pub ok: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseCheckDbBody {
    pub ok: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestAccountRegisterBody {
    pub public_key: Wrapper<PublicKey<F>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseAccountRegisterBody {
    pub address: Address<F>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestDepositAddBody {
    pub deposit_info: Vec<DepositInfo<F>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseDepositAddBody {
    pub ok: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestUserAssetDigestSetBody {
    pub address: Address<F>,
    pub user_asset_digest: WrappedHashOut<F>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseUserAssetDigestSetBody {
    pub proof: SparseMerkleProcessProof<GoldilocksHashOut, GoldilocksHashOut, GoldilocksHashOut>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestTxCreateBody {
    pub signer_address: Address<F>,
    // pub signer_nonce: u32,
    pub merge_witnesses: Vec<MergeProof<F>>,
    pub purge_input_witnesses: Vec<(SmtProcessProof<F>, SmtProcessProof<F>, SmtProcessProof<F>)>,
    pub purge_output_witnesses: Vec<(SmtProcessProof<F>, SmtProcessProof<F>, SmtProcessProof<F>)>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseTxCreateBody {
    pub user_tx_proof: MergeAndPurgeTransitionProofWithPublicInputs<F, C, D>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestTxSendBody {
    pub user_tx_proof: MergeAndPurgeTransitionProofWithPublicInputs<F, C, D>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseTxSendBody {
    pub tx_hash: GoldilocksHashOut,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestTxBroadcastBody {
    pub signer_address: Address<F>,
    pub tx_hash: WrappedHashOut<F>,
    pub nonce: WrappedHashOut<F>,
    pub purge_output_inclusion_witnesses: Vec<SmtInclusionProof<F>>,
    pub assets: Vec<Vec<Asset<F>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseTxBroadcastBody {
    pub ok: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBlockProposeBody {
    // pub user_tx_proofs: Vec<MergeAndPurgeTransitionProofWithPublicInputs<F, C, D>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBlockProposeBody {
    pub current_block_number: u32,
    pub new_world_state_root: GoldilocksHashOut,
    pub user_tx_proofs: Vec<MergeAndPurgeTransitionProofWithPublicInputs<F, C, D>>, // for debug
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestSignedDiffSendBody {
    pub tx_hash: GoldilocksHashOut,
    pub received_signature: SimpleSignatureProofWithPublicInputs<F, C, D>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseSignedDiffSendBody {
    pub ok: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBlockApproveBody {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBlockApproveBody {
    pub new_block: BlockInfo<F>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestTxReceivedQuery {
    pub user_address: Address<F>,
    pub since: Option<u32>,
    pub until: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseTxReceivedQuery {
    pub proofs: Vec<ReceivedAssetProof<F>>,
    pub latest_block_number: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBlockQuery {
    pub since: Option<u32>,
    pub until: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBlockQuery {
    pub blocks: Vec<BlockInfo<F>>,
    pub latest_block_number: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockDetails {
    pub block_number: u32,
    pub user_tx_proofs: Vec<MergeAndPurgeTransitionProofWithPublicInputs<F, C, D>>,
    pub received_signature_proofs: Vec<Option<SimpleSignatureProofWithPublicInputs<F, C, D>>>,
    pub world_state_process_proofs: Vec<SmtProcessProof<F>>,
    pub world_state_revert_proofs: Vec<SmtProcessProof<F>>,
    pub latest_account_process_proofs: Vec<SmtProcessProof<F>>,
    pub deposit_list: Vec<DepositInfo<F>>,
    pub block_headers_proof_siblings: Vec<WrappedHashOut<F>>,
    pub prev_block_header: BlockHeader<F>,
    pub default_simple_signature_proof: SimpleSignatureProofWithPublicInputs<F, C, D>,
    pub default_user_tx_proof: MergeAndPurgeTransitionProofWithPublicInputs<F, C, D>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBlockDetailQuery {
    pub block_number: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBlockDetailQuery {
    pub block_details: BlockDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestTxWitnessQuery {
    pub user_address: Address<F>,
    pub tx_hash: WrappedHashOut<F>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseTxWitnessQuery {
    // pub block_header: BlockHeader<F>,
    pub tx_inclusion_witness: MerkleProof<F>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUserAssetProofQuery {
    pub user_state_root: GoldilocksHashOut,
    pub user_address: Address<F>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseUserAssetProofBody {
    pub proof: SparseMerkleInclusionProof<K, V, I>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestLatestBlockQuery {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseLatestBlockQuery {
    pub block: BlockInfo<F>,
}
