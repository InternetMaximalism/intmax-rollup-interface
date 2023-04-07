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
    pub offer_manager_contract_address: &'a str,
    pub reverse_offer_manager_contract_address: &'a str,
}

pub const SCROLL_NETWORK_CONFIG: ContractConfig = ContractConfig {
    rpc_url: "https://alpha-rpc.scroll.io/l2",
    chain_id: 534353,
    offer_manager_contract_address: "0xD8f7FbABEacD6f103FB7278b3a7106e2fFBF0763",
    reverse_offer_manager_contract_address: "0x2D372972f8c325dEFD7252c0e7d8cBd09a8A4c67",
};

pub const POLYGON_NETWORK_CONFIG: ContractConfig = ContractConfig {
    rpc_url: "https://rpc.public.zkevm-test.net",
    chain_id: 1442,
    offer_manager_contract_address: "0x5602c213E1aEe9159E2A4d11fbFe19C56E7B3bE1",
    reverse_offer_manager_contract_address: "0xD9626E93c03E83647b1202a4a1CA96Bcc399F9E7",
};
