//! The lander as a Tor onion service. The onion address authenticates it, so requests are plain
//! HTTP inside Tor. A proof pays whoever submits its settlement, so the lander holds no role but
//! landing: anyone may land the same proof first, and a lander that does not take the submitter
//! fee is not used.

mod base64;
#[cfg(feature = "fuzzing")]
pub(crate) mod fuzz_hook;
mod handoff;
mod landers;
mod open;
mod reply;

pub(crate) use base64::encode;
use handoff::ask;
pub use handoff::{hand, state};
pub use landers::hand_first;
pub use reply::{Handed, RelayState};

#[cfg(test)]
#[path = "reply_hostile_test.rs"]
mod reply_hostile_test;
