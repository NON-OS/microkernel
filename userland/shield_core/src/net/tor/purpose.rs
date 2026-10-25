// NONOS Operating System (AGPL-3.0-or-later)
//! What a stream is for, as the phones' callers name it.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Purpose {
    Scan,
    Relay,
    Deposit,
    Mainnet,
    Sepolia,
}
