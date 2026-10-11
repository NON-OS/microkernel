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

//! 256 bytes to a policy, refusing anything `encode` would not have written.

use crate::error::PolicyError;
use crate::layout::*;
use crate::mask::KNOWN;
use crate::policy::LinuxAppPolicy;
use crate::syscall_set::SyscallSet;

pub fn decode(bytes: &[u8]) -> Result<LinuxAppPolicy, PolicyError> {
    let b: &[u8; LEN] = bytes.try_into().map_err(|_| PolicyError::Length)?;
    if b[AT_MAGIC..AT_VERSION] != MAGIC {
        return Err(PolicyError::Magic);
    }
    if u16::from_le_bytes([b[AT_VERSION], b[AT_VERSION + 1]]) != VERSION {
        return Err(PolicyError::Version);
    }
    let pad_head = &b[AT_PAD_HEAD..AT_PROGRAM];
    let pad_tail = &b[AT_PAD_TAIL..];
    if pad_head.iter().chain(pad_tail).any(|&x| x != 0) {
        return Err(PolicyError::Reserved);
    }
    let mask = u64::from_le_bytes(field(b, AT_MASK));
    if mask & !KNOWN != 0 {
        return Err(PolicyError::UnknownMask);
    }
    let syscall_set = SyscallSet::from_bytes(field(b, AT_SYSCALLS))?;
    Ok(LinuxAppPolicy {
        program_blake3: field(b, AT_PROGRAM),
        mask,
        listing_id: field(b, AT_LISTING),
        release_id: field(b, AT_RELEASE),
        net_dests_hash: field(b, AT_NET_DESTS),
        syscall_set,
    })
}

fn field<const N: usize>(b: &[u8; LEN], at: usize) -> [u8; N] {
    let mut out = [0u8; N];
    out.copy_from_slice(&b[at..at + N]);
    out
}
