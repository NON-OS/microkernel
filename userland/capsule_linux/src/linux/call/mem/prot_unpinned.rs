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

//! Which of mprotect's peer calls left the kernel's pins over their chunk
//! gone. Pure, so capsule_linux_proofs holds it.

/// The kernel's EFAULT: MkPeerProtect unpinned the chunk, then could not
/// change a page of it.
pub const PEER_FAULT: i64 = -14;

/// Whether the kernel unpinned every surface pinned over a chunk, from its
/// MkPeerProtect answer. It unpins once the guest and span pass its checks
/// and before it changes a page (peer_protect.rs), so a success or a fault
/// has unpinned the chunk and any other refusal has left it as it was.
pub fn chunk_unpinned(rc: i64) -> bool {
    rc >= 0 || rc == PEER_FAULT
}
