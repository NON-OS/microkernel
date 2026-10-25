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

//! A decoded policy.

use crate::syscall_set::SyscallSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinuxAppPolicy {
    /// BLAKE3 of the program's ELF, the bytes attest-before-run verifies.
    pub program_blake3: [u8; 32],
    /// What capsule_linux serves it (crate::mask).
    pub mask: u64,
    /// The market listing and release it was published under; zero for a
    /// program the vendor ships in the image.
    pub listing_id: [u8; 32],
    pub release_id: [u8; 32],
    /// BLAKE3("NONOS.linux.netdests.v1" | the destinations it may reach);
    /// zero means any, so long as the route is over the anonymity net. The
    /// whole 32 bytes: a prefix short enough to grind is no commitment.
    pub net_dests_hash: [u8; 32],
    /// The system calls it may complete.
    pub syscall_set: SyscallSet,
}
