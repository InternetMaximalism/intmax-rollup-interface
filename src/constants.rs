/// world state tree における user 層の tree の深さ
pub const N_LOG_MAX_USERS: usize = 24;

/// world state tree における user 層の tree の深さ
pub const N_LOG_MAX_TXS: usize = 24;

/// world state tree における user 層の tree の深さ
pub const N_LOG_MAX_CONTRACTS: usize = 8;

/// world state tree における user 層の tree の深さ
pub const N_LOG_MAX_VARIABLES: usize = 8;

/// diff tree における transaction 層の tree の深さ
pub const N_LOG_TXS: usize = 4;
pub const N_TXS: usize = 2usize.pow(N_LOG_TXS as u32);

/// diff tree における transaction 層の tree の深さ
pub const N_LOG_RECIPIENTS: usize = 8;

/// diff tree における transaction 層の tree の深さ
pub const N_LOG_CONTRACTS: usize = N_LOG_MAX_CONTRACTS;

/// diff tree における transaction 層の tree の深さ
pub const N_LOG_VARIABLES: usize = N_LOG_MAX_VARIABLES;

/// 1 つの block に含める deposit の数
pub const N_DEPOSITS: usize = 16;

/// 1 つの block に含める merge の数
pub const N_MERGES: usize = 16;

/// 1 つの block に含める purge の数
pub const N_DIFFS: usize = 16;

/// 1 つの batch でまとめる block の数
pub const N_BLOCKS: usize = 4;

/// block number の最大値の対数
pub const N_LOG_MAX_BLOCKS: usize = 32;
