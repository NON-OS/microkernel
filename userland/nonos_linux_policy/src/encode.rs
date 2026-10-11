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

//! A policy to its 256 bytes.

use crate::layout::*;
use crate::policy::LinuxAppPolicy;

impl LinuxAppPolicy {
    pub fn encode(&self) -> [u8; LEN] {
        let mut out = [0u8; LEN];
        out[AT_MAGIC..AT_VERSION].copy_from_slice(&MAGIC);
        out[AT_VERSION..AT_PAD_HEAD].copy_from_slice(&VERSION.to_le_bytes());
        out[AT_PROGRAM..AT_MASK].copy_from_slice(&self.program_blake3);
        out[AT_MASK..AT_LISTING].copy_from_slice(&self.mask.to_le_bytes());
        out[AT_LISTING..AT_RELEASE].copy_from_slice(&self.listing_id);
        out[AT_RELEASE..AT_NET_DESTS].copy_from_slice(&self.release_id);
        out[AT_NET_DESTS..AT_SYSCALLS].copy_from_slice(&self.net_dests_hash);
        out[AT_SYSCALLS..AT_PAD_TAIL].copy_from_slice(self.syscall_set.as_bytes());
        out
    }
}
