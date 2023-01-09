use intmax_zkp_core::config::RollupConstants;

pub const LOG_MAX_N_USERS: usize = 16;
pub const LOG_MAX_N_TXS: usize = 24;
pub const LOG_MAX_N_CONTRACTS: usize = LOG_MAX_N_USERS;
pub const LOG_MAX_N_VARIABLES: usize = 8;
pub const LOG_N_TXS: usize = 4;
pub const LOG_N_RECIPIENTS: usize = LOG_MAX_N_USERS;
pub const LOG_N_CONTRACTS: usize = LOG_MAX_N_CONTRACTS;
pub const LOG_N_VARIABLES: usize = LOG_MAX_N_VARIABLES;
pub const N_REGISTRATIONS: usize = 16;
pub const N_DEPOSITS: usize = 16;
pub const N_MERGES: usize = 16;
pub const N_DIFFS: usize = 16;
pub const N_BLOCKS: usize = 4;

/// block number の最大値の対数
pub const N_LOG_MAX_BLOCKS: usize = 32;

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
    n_blocks: N_BLOCKS,
};
