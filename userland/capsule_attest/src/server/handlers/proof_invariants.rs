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

//! The invariants, each with the result of actually testing it.
//!
//! Every record now carries a verdict and the count that produced it, appended
//! after the three text fields the wire format already had. A caller that only
//! reads the text sees what it always saw; a caller that reads to the end learns
//! whether the claim was checked and what was found.
//!
//! The verdict for a claim this capsule cannot settle is `V_UNCHECKED`, never a
//! pass. That distinction is the entire reason for the change: six claims served
//! as prose were indistinguishable from six claims that had been verified.

use super::proof_verdict::{evaluate, write_field};
use crate::protocol::{Request, E_INVAL, HDR_LEN, STATUS_LEN};
use crate::server::respond;
use crate::state::live::snapshot;
use crate::state::INVARIANTS;

// Three text fields, each 4-byte length-prefixed, then the verdict byte and the
// two counts behind it.
const RECORD_FIXED: usize = 12 + 1 + 8;

pub fn run(out: &mut [u8], req: &Request) -> usize {
    let snap = snapshot();
    let count = INVARIANTS.len() as u32;
    let dst = HDR_LEN + STATUS_LEN;
    if dst + 4 > out.len() {
        return respond::status(out, req, E_INVAL);
    }
    out[dst..dst + 4].copy_from_slice(&count.to_le_bytes());
    let mut written = 4usize;
    for inv in INVARIANTS {
        let needed = RECORD_FIXED + inv.name.len() + inv.claim.len() + inv.mechanism.len();
        if dst + written + needed > out.len() {
            return respond::status(out, req, E_INVAL);
        }
        written += write_field(&mut out[dst + written..], inv.name);
        written += write_field(&mut out[dst + written..], inv.claim);
        written += write_field(&mut out[dst + written..], inv.mechanism);
        let (verdict, found, of) = evaluate(inv.probe, snap.as_ref());
        let at = dst + written;
        out[at] = verdict;
        out[at + 1..at + 5].copy_from_slice(&found.to_le_bytes());
        out[at + 5..at + 9].copy_from_slice(&of.to_le_bytes());
        written += 9;
    }
    respond::with_payload(out, req, 0, written)
}
