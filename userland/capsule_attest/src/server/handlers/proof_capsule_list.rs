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

//! What is running, with the authority the kernel actually granted it.
//!
//! The wire shape is unchanged: a count, then `name_len | name | caps` per
//! capsule. What changed is where the bytes come from. They used to be a literal
//! list of seventeen names with masks typed beside them; they are now one read of
//! the kernel's process table, so the answer is what is running rather than what
//! was running when someone last edited this file.

use crate::protocol::{Request, E_INVAL, HDR_LEN, STATUS_LEN};
use crate::server::respond;
use crate::state::live::snapshot;

// E_AGAIN would be wrong here: the kernel refusing the process table is not a
// transient condition the caller should retry through, and answering 0 capsules
// would be a lie about an empty machine.
pub fn run(out: &mut [u8], req: &Request) -> usize {
    let Some(snap) = snapshot() else {
        return respond::status(out, req, E_INVAL);
    };
    let dst = HDR_LEN + STATUS_LEN;
    if dst + 4 > out.len() {
        return respond::status(out, req, E_INVAL);
    }

    // The count is written last, once it is known how many entries actually fit,
    // so a caller can never read a count that promises more than the payload
    // carries.
    let mut written = 4usize;
    let mut listed = 0u32;
    for capsule in snap.live() {
        let name = capsule.name();
        let needed = 4 + name.len() + 8;
        if dst + written + needed > out.len() {
            break;
        }
        let at = dst + written;
        out[at..at + 4].copy_from_slice(&(name.len() as u32).to_le_bytes());
        out[at + 4..at + 4 + name.len()].copy_from_slice(name);
        let caps_at = at + 4 + name.len();
        out[caps_at..caps_at + 8].copy_from_slice(&capsule.caps.to_le_bytes());
        written += needed;
        listed += 1;
    }
    out[dst..dst + 4].copy_from_slice(&listed.to_le_bytes());
    respond::with_payload(out, req, 0, written)
}
