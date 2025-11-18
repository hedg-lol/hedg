//! Protocol constants and configuration values.
//!
//! These values are compiled into the program binary and
//! cannot be changed without a program upgrade.

pub const MIN_COLLATERAL: u64 = 1_000_000_000;
pub const SAFE_PERIOD: i64 = 86_400;
pub const PROTOCOL_FEE_BPS: u64 = 50;
pub const TRADE_FEE_BPS: u64 = 100;
pub const DEPLOY_FEE: u64 = 20_000_000;
pub const ESCROW_SEED: &[u8] = b"escrow";
pub const CURVE_SEED: &[u8] = b"curve";
pub const RECORD_SEED: &[u8] = b"record";
