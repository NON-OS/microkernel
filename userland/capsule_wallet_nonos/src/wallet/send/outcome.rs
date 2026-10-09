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

//! What a broadcast came to, judged once and never retried.
//!
//! A signed transaction goes to the node exactly once. When the exchange
//! broke after the request had begun to go, nobody here can know whether
//! the node took it, and sending it again could only ever be answered by
//! the same transaction twice. So that case is said as unknown and the
//! receipt is followed under the hash computed here from the signed bytes,
//! which is the transaction's own whatever the node says. Pure, so
//! wallet_proofs holds the rule.

/// What the broadcast's exchange carried back.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Carried {
    /// The node answered: the hash it named when it took the transaction,
    /// None when it refused it without a reason.
    Answer(Option<[u8; 32]>),
    /// The node refused it, and why.
    Refused(&'static str),
    /// Nothing of the request went, and why.
    NotSent(&'static str),
    /// The exchange broke once the request had begun to go.
    Unknown,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Broadcast {
    /// Taken; the receipt is followed under this hash.
    Sent([u8; 32]),
    /// Perhaps taken; the receipt is followed, and nothing is sent again.
    Unknown([u8; 32]),
    /// The node answered and did not take it, and why.
    Refused(&'static str),
    /// Never reached the node.
    NotSent(&'static str),
}

impl Broadcast {
    /// The hash to follow a receipt under, when the transaction may be on
    /// its way into a block.
    pub fn follow(&self) -> Option<[u8; 32]> {
        match *self {
            Broadcast::Sent(h) | Broadcast::Unknown(h) => Some(h),
            Broadcast::Refused(_) | Broadcast::NotSent(_) => None,
        }
    }
}

/// A refusal the node gave no reason for.
pub const REFUSED: &str = "The node refused the transaction without saying why.";

/// The broadcast's result for the transaction whose own hash is `local`.
/// The receipt is always followed under `local`: a node naming another
/// hash cannot point the wallet at someone else's transaction.
pub fn judge(carried: Carried, local: &[u8; 32]) -> Broadcast {
    match carried {
        Carried::Answer(Some(_)) => Broadcast::Sent(*local),
        Carried::Answer(None) => Broadcast::Refused(REFUSED),
        Carried::Refused(why) => Broadcast::Refused(why),
        Carried::NotSent(why) => Broadcast::NotSent(why),
        Carried::Unknown => Broadcast::Unknown(*local),
    }
}
