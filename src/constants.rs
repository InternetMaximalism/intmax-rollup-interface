use intmax_zkp_core::config::RollupConstants;

const LOG_MAX_N_USERS: usize = 16;
const LOG_MAX_N_TXS: usize = 24;
const LOG_MAX_N_CONTRACTS: usize = LOG_MAX_N_USERS;
const LOG_MAX_N_VARIABLES: usize = 8;
const LOG_N_TXS: usize = 4;
const LOG_N_RECIPIENTS: usize = LOG_MAX_N_USERS;
const LOG_N_CONTRACTS: usize = LOG_MAX_N_CONTRACTS;
const LOG_N_VARIABLES: usize = LOG_MAX_N_VARIABLES;
const N_REGISTRATIONS: usize = 16;
const N_DEPOSITS: usize = 16;
const N_SCROLL_FLAGS: usize = 16;
const N_POLYGON_FLAGS: usize = 16;
const N_MERGES: usize = 16;
const N_DIFFS: usize = 16;
const N_BLOCKS: usize = 4;

/// block number の最大値の対数
pub const LOG_MAX_N_BLOCKS: usize = 32;

pub const ROLLUP_CONSTANTS: RollupConstants = RollupConstants {
    log_max_n_users: LOG_MAX_N_USERS,
    log_max_n_txs: LOG_MAX_N_TXS,
    log_max_n_contracts: LOG_MAX_N_CONTRACTS,
    log_max_n_variables: LOG_MAX_N_VARIABLES,
    log_n_txs: LOG_N_TXS,
    log_n_recipients: LOG_N_RECIPIENTS,
    log_n_contracts: LOG_N_CONTRACTS,
    log_n_variables: LOG_N_VARIABLES,
    n_registrations: N_REGISTRATIONS,
    n_diffs: N_DIFFS,
    n_merges: N_MERGES,
    n_deposits: N_DEPOSITS,
    n_scroll_flags: N_SCROLL_FLAGS,
    n_polygon_flags: N_POLYGON_FLAGS,
    n_blocks: N_BLOCKS,
};

#[derive(Clone)]
pub struct ContractConfig<'a> {
    pub rpc_url: &'a str,
    pub chain_id: u64,
    pub verifier_contract_address: &'a str,
    pub offer_manager_contract_address: &'a str,
    pub reverse_offer_manager_contract_address: &'a str,
}

pub const DEPLOYER: &str = "0x8f68aE37a8339c8cD120187D41a284921F73feBE";

pub const SCROLL_ALPHA_NETWORK_CONFIG: ContractConfig = ContractConfig {
    rpc_url: "https://alpha-rpc.scroll.io/l2",
    chain_id: 534353,
    verifier_contract_address: "",
    offer_manager_contract_address: "0x007c969728eE4f068ceCF3405D65a037dB5BeEa1",
    reverse_offer_manager_contract_address: "0x4ee8cB7864df06A8c7703988C15bAaAB9ac47CAe",
};

pub const POLYGON_ZKEVM_TEST_NETWORK_CONFIG: ContractConfig = ContractConfig {
    rpc_url: "https://rpc.public.zkevm-test.net",
    chain_id: 1442,
    verifier_contract_address: "",
    offer_manager_contract_address: "0x161a72Bc1b76586a36A9014Dd58d401eE2B24094",
    reverse_offer_manager_contract_address: "0x1E316b313de98C7eCb2393995ef27715E3E1c7a7",
};

pub const LOCAL_NETWORK_CONFIG: ContractConfig = ContractConfig {
    rpc_url: "http://localhost:8545",
    chain_id: 31337,
    verifier_contract_address: "0x610178dA211FEF7D417bC0e6FeD39F05609AD788",
    offer_manager_contract_address: "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0",
    reverse_offer_manager_contract_address: "0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9",
};
