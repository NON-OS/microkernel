//! A spend from this wallet on the launch pool: a private transfer to another
//! wallet, or a withdrawal to an EVM address, proved here and handed to a
//! relayer to settle. The relayer is paid in the proof, so the sender's own
//! address never appears on chain.

mod build;
mod flow;
pub(crate) mod not_before;
mod order;
mod pick;
pub(crate) mod ripe;
#[cfg(test)]
#[path = "ripe_test.rs"]
mod ripe_test;

pub use flow::{spend, SpendOutcome};
pub use order::{Destination, Order};

#[cfg(test)]
#[path = "spend_test.rs"]
mod spend_test;
