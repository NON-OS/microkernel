//! One account of the wallet as a list row shows it. Names are the shell's to show, by number.

/// An account: its number, the address of its public account, and whether it is active.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AccountSummary {
    pub index: u32,
    pub public_address: String,
    pub active: bool,
}

/// A view-only account: its number among them, which key it came from, and what it can see.
/// An incoming key counts every received note, spent or not, so its total is what arrived.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WatchedSummary {
    pub index: u32,
    pub kind: crate::keys::ViewKind,
    pub balances: Vec<crate::store::Balance>,
}
