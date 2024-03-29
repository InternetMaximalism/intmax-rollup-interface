use intmax_zkp_core::{
    merkle_tree::tree::MerkleProof,
    plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig},
    rollup::{
        address_list::TransactionSenderWithValidity, block::BlockInfo,
        gadgets::deposit_block::DepositInfo,
    },
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
use serde::{Deserialize, Serialize};

type K = GoldilocksHashOut;
type V = GoldilocksHashOut;
type I = GoldilocksHashOut;

type C = PoseidonGoldilocksConfig;
// type H = <C as GenericConfig<D>>::InnerHasher;
type F = <C as GenericConfig<D>>::F;
const D: usize = 2;

pub const AGGREGATOR_NAME: &str = "intmax-rollup";

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBool {
    pub ok: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseCheckHealth {
    pub name: String,
    pub version: String,
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
pub struct RequestTxConfirmationWitnessQuery {
    pub tx_hash: GoldilocksHashOut,
    pub recipient: Address<F>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseTxConfirmationWitnessQuery {
    pub witness: String,
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
pub struct RequestAssetReceivedQuery {
    pub user_address: Address<F>,
    pub since: Option<u32>,
    pub until: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseAssetReceivedQuery {
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
    pub scroll_flag_list: Vec<DepositInfo<F>>,
    pub polygon_flag_list: Vec<DepositInfo<F>>,
    pub block_headers_proof_siblings: Vec<WrappedHashOut<F>>,
    pub prev_block_header: BlockHeader<F>,
    pub block_header: BlockHeader<F>,
    pub default_simple_signature_proof: SimpleSignatureProofWithPublicInputs<F, C, D>,
    pub default_user_tx_proof: MergeAndPurgeTransitionProofWithPublicInputs<F, C, D>,
}

impl From<BlockDetails> for BlockInfo<F> {
    fn from(value: BlockDetails) -> Self {
        Self {
            header: value.block_header,
            transactions: value
                .user_tx_proofs
                .iter()
                .map(|v| v.public_inputs.tx_hash)
                .collect::<Vec<_>>(),
            deposit_list: value.deposit_list,
            scroll_flag_list: value.scroll_flag_list,
            polygon_flag_list: value.polygon_flag_list,
            address_list: value
                .latest_account_process_proofs
                .iter()
                .map(|v| TransactionSenderWithValidity {
                    sender_address: Address::from_hash_out(*v.new_key),
                    is_valid: v.new_value == WrappedHashOut::from_u32(value.block_number),
                })
                .collect::<Vec<_>>(),
        }
    }
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
pub struct RequestTxReceiptQuery {
    pub user_address: Address<F>,
    pub tx_hash: WrappedHashOut<F>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseTxReceiptQuery {
    pub user_asset_inclusion_witness: SparseMerkleInclusionProof<K, V, I>,
    pub tx_inclusion_witness: MerkleProof<F>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseTxWitnessQuery {
    // pub block_header: BlockHeader<F>,
    pub tx_inclusion_witness: MerkleProof<F>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUserAssetProofQuery {
    pub world_state_digest: GoldilocksHashOut, // unused
    pub user_address: Address<F>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseUserAssetProofBody {
    pub proof: SparseMerkleInclusionProof<K, V, I>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestTransactionProofQuery {
    pub tx_hash: WrappedHashOut<F>,
    pub receiver_address: Address<F>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TxDetailGoldilocks {
    pub tx_hash: WrappedHashOut<F>,
    pub nonce: WrappedHashOut<F>,
    pub signer_address: Address<F>,
    pub receiver_address: Address<F>,
    pub inclusion_witness: SmtInclusionProof<F>,
    pub assets: Vec<Asset<F>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseTransactionProofQuery {
    pub tx_details: TxDetailGoldilocks,
    pub transaction_proof: MerkleProof<F>,
    pub block_header: BlockHeader<F>,
    pub witness: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestLatestBlockQuery {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseLatestBlockQuery {
    pub block: BlockInfo<F>,
}
