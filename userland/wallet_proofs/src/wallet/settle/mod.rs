// NONOS Operating System (AGPL-3.0-or-later)
//! The wallet's settle client, over a fake port (send.rs): call.rs and
//! heard.rs are the capsule's own.

#[path = "../../../../capsule_wallet_nonos/src/wallet/settle/call.rs"]
mod call;
#[path = "../../../../capsule_wallet_nonos/src/wallet/settle/heard.rs"]
mod heard;
pub mod send;

pub use call::SettleCall;
pub use send::{ESCROW, LANDED};
