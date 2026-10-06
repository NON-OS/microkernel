//! The wallet: what the shells drive. Everything that decides anything lives
//! here or below, so a shell renders state and forwards intent and nothing
//! else.

pub(crate) mod abi;
pub mod anchor;
mod deposit;
mod deposit_gate;
mod deposit_read;
pub mod deposit_split;
pub mod handoff;
pub mod history;
pub(crate) mod history_join;
mod intent_limits;
pub(crate) mod intent_rules;
pub mod kept;
mod one_call;
mod paths;
pub mod publish;
mod recover;
pub(crate) mod registry;
pub(crate) mod relay_body;
pub mod rewards;
pub(crate) mod scan_history;
mod session;
mod settle;
pub(crate) mod settle_self;
pub mod spend;
mod sync_chain;
pub(crate) mod take_back;
pub(crate) mod watch;

pub use deposit::{absorb_calldata, prepare_deposit, split_deposit, DepositRequest, NATIVE_ASSET};
pub use deposit_gate::{check as check_deposit, PoolState, Refusal};
pub use deposit_read::{read_pool_state, read_pool_state_checked};
pub use intent_rules::{check_intent, Intent, IntentRefusal, PoolRules, MAX_FEE_BPS};
pub use one_call::one_call;
pub use paths::Paths;
pub use recover::{beta_refund_calldata, claim_calldata, commit_root_calldata, read_recoverable};
pub use scan_history::{scan_history, scan_history_as, sync_chain};
pub use session::book::MAX_ACCOUNTS;
pub use session::Session;
pub use settle::settle_calldata;
pub use sync_chain::{fetch_history, fetch_history_until, History};
