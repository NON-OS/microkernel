//! One entry of the history as shown, and where it stands.

/// Where one entry of the history stands.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stage {
    /// Sent or proved, and not seen on chain yet.
    OnItsWay,
    /// Its owner sent the settlement and it is not seen on chain yet.
    Settling,
    /// In the pool: a deposit stored, a spend landed, a payment received.
    Done,
    /// Taken back: it never landed and its notes are spendable again.
    TakenBack,
}

/// One entry of the history, oldest first.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Entry {
    /// `DEPOSIT_SENT`, `SENT`, `WITHDRAWN` or `RECEIVED`: what the entry is.
    pub kind: u8,
    pub asset_id: u64,
    pub value: u64,
    pub at: u64,
    pub stage: Stage,
    /// The transaction that put it on chain, when one is known.
    pub tx: Option<[u8; 32]>,
}
