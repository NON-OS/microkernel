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

//! What the publisher screen says of the send it follows, as the send
//! ledger's fate reads, and of a BROADCAST answer naming another hash.
//! Only a send that may still land is said to be sent.

use alloc::format;
use alloc::string::String;

use super::super::super::hex_id::id;
use crate::wallet::send::sent::Fate;

/// the project for the send followed under `hash` (the one signed), and
/// `more`, the calls after it, only while it may still land.
pub fn said(hash: &[u8; 32], fate: Option<Fate>, more: &str) -> String {
    let h = id(hash);
    let (line, more) = match fate {
        Some(Fate::Replaced) => (
            format!(
                "Replaced: another transaction from this account took the same nonce, so \
                 {h} did not take effect, and nothing after it follows. Cancel to leave."
            ),
            "",
        ),
        Some(Fate::Dropped) => (
            format!(
                "Dropped: the network dropped {h} and no block has it, so nothing was \
                 sent. Only if its nonce reads unused is it offered again for Sign; \
                 otherwise Cancel."
            ),
            "",
        ),
        Some(Fate::InBlock { ok: false, .. } | Fate::Confirmed { ok: false }) => {
            (format!("Reverted: {h} is in a block and did not take effect; its fee was spent."), "")
        }
        Some(_) => (format!("Sent {h}; waiting for it to land."), more),
        None => (format!("Handed to settle as {h}; the send ledger no longer holds its fate."), ""),
    };
    format!("{line}{more} Its own hash, the one followed:")
}

/// The node's BROADCAST answer, when it is not the hash followed: said on
/// its own line, as a warning, and never followed.
pub fn echo_said(echo: Option<[u8; 32]>) -> Option<String> {
    let e = echo?;
    Some(format!(
        "Warning: the node answered {}, which is not this transaction's own hash; \
         the wallet follows its own, above, never the node's.",
        id(&e)
    ))
}
