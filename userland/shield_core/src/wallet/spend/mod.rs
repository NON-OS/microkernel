//! A spend from this wallet on the launch pool: a private transfer to another
//! wallet, or a withdrawal to an EVM address, proved here and handed to a
//! relayer to settle. The relayer is paid in the proof, so the sender's own
//! address never appears on chain.

mod build;
mod flow;
pub(crate) mod not_before;
mod pick;
pub(crate) mod ripe;
#[cfg(test)]
#[path = "ripe_test.rs"]
mod ripe_test;

pub use flow::{spend, SpendOutcome};

/// Where the money goes.
pub enum Destination {
    /// Another wallet: its spend key, and the key its note is sealed to.
    Wallet { spend_pk: [u64; 4], sealed_to: Box<[u8; crate::keys::ENCAPSULATION_KEY_BYTES]> },
    /// Out of the pool, to an EVM address.
    Withdraw { recipient: [u8; 20] },
}

/// One spend: of which asset, where, how much, and who is paid for landing
/// its settlement.
pub struct Order {
    pub asset: crate::net::asset::Asset,
    pub to: Destination,
    /// In the asset's note units, as is the fee.
    pub amount: u64,
    pub fee: u64,
    pub fee_to: [u8; 20],
    /// The earliest time the pool settles it, in seconds, on the ten-minute grid.
    pub not_before: u64,
    /// The owner typed the confirmation to spend notes that have not waited.
    pub early: bool,
}

#[cfg(test)]
#[path = "spend_test.rs"]
mod spend_test;
