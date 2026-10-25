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

//! What a node's refusal of a broadcast said, in words a holder can act on.
//!
//! The node answers with free text. The errors nodes give for a raw
//! transaction are a small set, so each is named here as its own sentence,
//! and one the wallet does not know is said as that, never as another.

/// What the node's error came to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeSaid {
    /// The node already holds this very transaction: it is on its way.
    AlreadyKnown,
    /// Refused, and why.
    Refused(&'static str),
}

pub const UNNAMED: &str = "The node refused the transaction without a reason this wallet knows.";

/// The refusal in a broadcast's error reply.
pub fn broadcast_error(resp: &[u8]) -> NodeSaid {
    let said = message(resp);
    let has = |needle: &str| contains(said, needle.as_bytes());
    if has("already known") || has("known transaction") {
        return NodeSaid::AlreadyKnown;
    }
    let why = if has("insufficient funds") {
        "The account does not hold enough ETH for the amount and the network fee."
    } else if has("nonce too low") {
        "Another transaction from this account already used this nonce. Review it again."
    } else if has("nonce too high") {
        "The node is behind on this account. Wait for the last transaction, then review again."
    } else if has("underpriced") {
        "The node wants a higher fee for this transaction. Review it again for a fresh fee."
    } else if has("less than block base fee") {
        "The network fee rose past this transaction's cap. Review it again for a fresh fee."
    } else if has("intrinsic gas too low") || has("gas limit") {
        "The node refused this transaction's gas limit. Review it again."
    } else {
        UNNAMED
    };
    NodeSaid::Refused(why)
}

/// The error's message, or the whole reply when it names none.
fn message(resp: &[u8]) -> &[u8] {
    let pat = b"\"message\":\"";
    let Some(at) = resp.windows(pat.len()).position(|w| w == pat) else { return resp };
    let start = at + pat.len();
    let end = resp[start..].iter().position(|b| *b == b'"').map_or(resp.len(), |n| start + n);
    &resp[start..end]
}

/// Whether `hay` holds `needle`, ignoring ASCII case.
fn contains(hay: &[u8], needle: &[u8]) -> bool {
    hay.windows(needle.len()).any(|w| w.eq_ignore_ascii_case(needle))
}
