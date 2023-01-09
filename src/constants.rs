use intmax_zkp_core::config::RollupConstants;

/// block number の最大値の対数
pub const N_LOG_MAX_BLOCKS: usize = 32;

pub const ROLLUP_CONSTANTS: RollupConstants = RollupConstants {
    log_max_n_users: 3,
    log_max_n_txs: 3,
    log_max_n_contracts: 3,
    log_max_n_variables: 3,
    log_n_txs: 2,
    log_n_recipients: 3,
    log_n_contracts: 3,
    log_n_variables: 3,
    n_registrations: 2,
    n_diffs: 2,
    n_merges: 2,
    n_deposits: 2,
    n_blocks: 2,
};
