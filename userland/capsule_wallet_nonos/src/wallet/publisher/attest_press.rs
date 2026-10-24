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

//! What a press does to an attest: a row only opens its review, and only the
//! review's own Sign signs it. Pure, so the wallet proofs run it on the host.

use crate::wallet::arm::rows::{ROW_ATTEST, ROW_INTENT};
use crate::wallet::screen::hits::Press;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AttestAct {
    /// Show release i's attest for review; nothing is signed.
    Open(usize),
    /// Sign release i's attest, as reviewed.
    Sign(usize),
    /// Leave the review; nothing is signed.
    Cancel,
    /// Not an attest press.
    Other,
}

/// `press` while release `asked`'s review shows, or none does.
pub fn attest_act(asked: Option<usize>, press: Press) -> AttestAct {
    match (asked, press) {
        (Some(i), Press::Footer(0)) => AttestAct::Sign(i),
        (Some(_), Press::Footer(1) | Press::Back) => AttestAct::Cancel,
        (Some(_), _) => AttestAct::Other,
        (None, Press::Row(r)) if (ROW_ATTEST..ROW_INTENT).contains(&r) => {
            AttestAct::Open((r - ROW_ATTEST) as usize)
        }
        (None, _) => AttestAct::Other,
    }
}

/// Sign may sign only the digest its review showed, and only while the
/// attest still has it: none shown, or none now, signs nothing.
pub fn may_sign(shown: Option<[u8; 32]>, now: Option<[u8; 32]>) -> bool {
    matches!((shown, now), (Some(a), Some(b)) if a == b)
}
