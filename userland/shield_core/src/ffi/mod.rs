//! The boundary the shells see. It is thin by design: every method here
//! either reads state the core already decided or hands work to a module that
//! decides it. Nothing in Kotlin or Swift gets to make a choice about money.

mod account_list;
mod account_types;
mod amount;
mod amount_read;
mod cancel;
mod chain_types;
mod evm;
mod parts;
mod preview;
mod seconds;
mod shield_types;
mod wallet;

pub use account_list::{AccountSummary, WatchedSummary};
pub use account_types::{PublicBalances, PublicReview, PublicSent, PublicSwap};
pub use amount::{format_amount, max_note_amount, DECIMALS};
pub use amount_read::{amount_unit, parse_amount};
pub use cancel::CancelToken;
pub use chain_types::{ChainSummary, DepositTicket, Recoverable, SpendTicket};
pub use parts::AddressParts;
pub use preview::{address_of_key, address_of_words};
pub use seconds::format_seconds;
pub use shield_types::PublicShield;
pub use wallet::Wallet;
