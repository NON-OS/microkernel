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

//! Reprotecting a span, a peer call at a time.

use nonos_libc::peer::mk_peer_protect;

use crate::linux::guest::{peer_prot, Guest, MAX_SPAN};

use super::prot::{PROT_ANY, PROT_EXEC, PROT_WRITE};
use super::prot_unpinned::chunk_unpinned;

/// Set the protection of a span already mapped in the guest, and record it
/// on the spans it covers.
pub fn protect_span(guest: &mut Guest, addr: u64, span: u64, prot: u64) -> i64 {
    let (write, exec, access) =
        (prot & PROT_WRITE != 0, prot & PROT_EXEC != 0, prot & PROT_ANY != 0);
    let bits = peer_prot(write, exec, access);
    let mut done = 0;
    while done < span {
        let take = (span - done).min(MAX_SPAN);
        let rc = mk_peer_protect(guest.pid, addr + done, take, bits);
        // The kernel unpins a surface pinned over a chunk before it changes
        // it (peer_protect.rs): the bridge shows none of those pins again,
        // and keeps those of a chunk the kernel refused before unpinning,
        // so it can still give them up itself.
        if chunk_unpinned(rc) {
            crate::linux::wayland::unmapped(&mut guest.scene, addr + done, take);
        }
        if rc < 0 {
            return rc;
        }
        done += take;
    }
    guest.set_prot(addr, span, write, exec, access);
    0
}
