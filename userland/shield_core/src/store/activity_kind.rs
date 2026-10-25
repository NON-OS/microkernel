//! One row of the history: the kinds a row may be, and the fields it carries.

/// A deposit sent from the public account: its note's commitment, its transaction.
pub const DEPOSIT_SENT: u8 = 1;
/// A deposit's note seen stored in the pool, by its commitment.
pub const DEPOSIT_IN: u8 = 2;
/// A private payment proved, by its first nullifier, with its change commitment.
pub const SENT: u8 = 3;
/// A withdrawal proved, the same.
pub const WITHDRAWN: u8 = 4;
/// A spend seen on chain, by its first nullifier, with its settlement when named.
pub const LANDED: u8 = 5;
/// A spend's settlement sent by its owner from the public account.
pub const SETTLED_SELF: u8 = 6;
/// A spend's notes taken back: it never lands.
pub const TAKEN_BACK: u8 = 7;
/// A note received from someone else, by its commitment.
pub const RECEIVED: u8 = 8;

/// One thing done, as a row writes it. Unused fields are zero.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Activity {
    pub kind: u8,
    pub asset_id: u64,
    pub value: u64,
    /// Seconds since 1970 when it was recorded.
    pub at: u64,
    pub tx: [u8; 32],
    /// The spend's first nullifier.
    pub tag: [u8; 32],
    /// A note's commitment: the deposit's, the change's, or the one received.
    pub cm: [u64; 4],
}

impl Activity {
    pub fn new(kind: u8, at: u64) -> Activity {
        Activity { kind, asset_id: 0, value: 0, at, tx: [0; 32], tag: [0; 32], cm: [0; 4] }
    }

    pub fn known(kind: u8) -> bool {
        (DEPOSIT_SENT..=RECEIVED).contains(&kind)
    }
}
