// NONOS Operating System (AGPL-3.0-or-later)
//! The wallet's settle client, over a fake port (send.rs): call.rs and
//! heard.rs are the capsule's own.

#[path = "../../../../capsule_wallet_nonos/src/wallet/settle/call.rs"]
mod call;
pub mod send;

/* heard.rs is mounted once, at the crate root, and named here as the
 * capsule names it so call.rs reaches it as `super::heard`. */
use crate::settle_heard as heard;

pub use call::SettleCall;
pub use send::{ESCROW, LANDED};
