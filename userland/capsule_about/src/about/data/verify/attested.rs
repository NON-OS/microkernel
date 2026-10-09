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

//! The capsules running now that the kernel admitted at spawn, from the
//! attestation registry, each with the name the process table gives its pid.
//! Reading the registry needs AttestRead; without it the kernel refuses, and
//! the screen says so rather than showing an empty list.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::{mk_attest_entries, ATTEST_ENTRY_LEN, PROC_NAME_LEN};

use super::names::Names;

/// The kernel's registry holds at most this many capsules.
const MAX_ENTRIES: usize = 256;

/// One registry entry. `authority` is 0 for the vendor's tree, 1 to 254 for a
/// root the user enrolled on this machine, and 255 for a publisher's signature
/// with no proof behind it.
#[derive(Clone, Copy)]
pub struct Attested {
    pub pid: u32,
    pub measurement: [u8; 32],
    pub authority: u8,
    pub name: [u8; PROC_NAME_LEN],
    pub name_len: usize,
}

/// In pid order, as the kernel folds them. `None` when the kernel refused the
/// read or answered a length that is not a whole number of entries.
pub fn attested() -> Option<Vec<Attested>> {
    let mut buf = vec![0u8; MAX_ENTRIES * ATTEST_ENTRY_LEN];
    let rc = mk_attest_entries(&mut buf);
    if rc < 0 || rc as usize > buf.len() || rc as usize % ATTEST_ENTRY_LEN != 0 {
        return None;
    }
    let names = Names::read();
    Some(buf[..rc as usize].chunks_exact(ATTEST_ENTRY_LEN).map(|e| entry(e, &names)).collect())
}

/* Pid big-endian, then the measurement, the mask, and the authority byte. */
fn entry(e: &[u8], names: &Names) -> Attested {
    let pid = u32::from_be_bytes([e[0], e[1], e[2], e[3]]);
    let mut measurement = [0u8; 32];
    measurement.copy_from_slice(&e[4..36]);
    let (name, name_len) = names.of(pid);
    Attested { pid, measurement, authority: e[ATTEST_ENTRY_LEN - 1], name, name_len }
}
