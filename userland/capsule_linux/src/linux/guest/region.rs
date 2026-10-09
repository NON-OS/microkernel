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

//! One span of a guest's address space, as this capsule laid it down.

use nonos_libc::peer::{PEER_PROT_EXEC, PEER_PROT_NONE, PEER_PROT_WRITE};

#[derive(Clone, Copy)]
pub struct Region {
    pub at: u64,
    pub len: u64,
    pub write: bool,
    pub exec: bool,
    /// False for PROT_NONE: the guest may not touch the span at all. A backed
    /// span keeps its pages and their bytes, present to the kernel only.
    pub access: bool,
    /// File bytes mapped without exec, so never proved: mprotect may not
    /// make them executable later.
    pub unproven: bool,
    /// False for a PROT_NONE reservation: address space taken, no frames.
    /// The kernel fills no page for a guest on its own, so a touch of one is a
    /// fault; a commit maps the part asked for and records it backed.
    pub backed: bool,
    /// Bytes Linux would give back after MADV_DONTNEED, not zero: a file's,
    /// an ELF segment's or a shared mapping's. This capsule cannot give them
    /// back, so it refuses that advice here.
    pub kept: bool,
}

impl Region {
    /// The protection the kernel is asked to give this span's pages.
    pub fn peer_prot(&self) -> u64 {
        peer_prot(self.write, self.exec, self.access)
    }
}

/// Peer protection bits for an access, a write and an exec permission.
pub fn peer_prot(write: bool, exec: bool, access: bool) -> u64 {
    if !access {
        return PEER_PROT_NONE;
    }
    let mut prot = 0;
    if write {
        prot |= PEER_PROT_WRITE;
    }
    if exec {
        prot |= PEER_PROT_EXEC;
    }
    prot
}
