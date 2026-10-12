// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//! Every transaction this wallet sent, per network and account, until the
//! chain has settled it. It gives the next nonce a floor, so a second
//! payment can never take the first's nonce from a node that has not seen
//! the first yet. It holds back what a waiting payment may still spend. And
//! it says what became of each payment: in a block, confirmed under enough
//! blocks, replaced by another with its nonce, or dropped without ever
//! landing. It is kept across a network switch, so leaving and coming back
//! forgets nothing. Pure: wallet_proofs drives it with readings of its own.

use alloc::vec::Vec;

/// Blocks on top of a payment's own, its included, before it is said to be
/// confirmed: past any reorganisation either network has seen since the merge.
pub const CONFIRMATIONS: u64 = 12;
/// How long a payment no node holds, and no block has, is waited for before
/// it is said to be dropped and its nonce is free again.
pub const DROP_MS: i64 = 30 * 60 * 1000;
/// Settled entries kept, newest first, for the screens; waiting ones are
/// always kept.
const SETTLED_KEPT: usize = 16;
/// Readings in a row that show its nonce used and no receipt before it is
/// said to be replaced: one node's receipt index can lag its blocks.
pub const REPLACED_AFTER: u8 = 3;

#[path = "sent_fate.rs"]
mod sent_fate;
#[path = "sent_judge.rs"]
mod sent_judge;
#[path = "sent_ledger.rs"]
mod sent_ledger;
pub use sent_fate::{fate, Fate, Reading};
pub use sent_ledger::usable;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Sent {
    pub chain: u64,
    pub from: [u8; 20],
    pub nonce: u64,
    pub hash: [u8; 32],
    pub at_ms: i64,
    /// The most ETH it can take: its value and its fee cap.
    pub eth_cost: u128,
    /// The token it moves, as the send screen numbers assets, and how much.
    pub token: Option<(u8, u128)>,
    pub fate: Fate,
    /// Readings in a row with its nonce used and no receipt.
    pub misses: u8,
    /// The broadcast broke off before the node said it took it.
    pub unknown: bool,
}

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct Ledger {
    pub list: Vec<Sent>,
}
