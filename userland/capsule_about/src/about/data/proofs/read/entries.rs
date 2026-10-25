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

//! The attestation registry's entries, and a network holder named from them.

use super::super::census::Admitted;
use super::snapshot::{Holder, Row};
use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::{mk_attest_entries, ATTEST_ENTRY_LEN, PROC_NAME_LEN};

const MAX_ENTRIES: usize = 256;

/* Pid big-endian, the measurement, the mask big-endian, the authority byte. */
pub(super) fn entries() -> Option<Vec<(Admitted, [u8; 32])>> {
    let mut buf = vec![0u8; MAX_ENTRIES * ATTEST_ENTRY_LEN];
    let rc = mk_attest_entries(&mut buf);
    if rc < 0 || rc as usize > buf.len() || rc as usize % ATTEST_ENTRY_LEN != 0 {
        return None;
    }
    Some(
        buf[..rc as usize]
            .chunks_exact(ATTEST_ENTRY_LEN)
            .map(|e| {
                let mut m = [0u8; 32];
                m.copy_from_slice(&e[4..36]);
                let mut caps = [0u8; 8];
                caps.copy_from_slice(&e[36..44]);
                let a = Admitted {
                    pid: u32::from_be_bytes([e[0], e[1], e[2], e[3]]),
                    caps: u64::from_be_bytes(caps),
                    authority: e[ATTEST_ENTRY_LEN - 1],
                };
                (a, m)
            })
            .collect(),
    )
}

pub(super) fn holder(a: &Admitted, measurement: &[u8; 32], rows: &[Row]) -> Holder {
    let (name, name_len) = match rows.iter().find(|r| r.pid == a.pid) {
        Some(r) => (r.name, r.len),
        None => ([0u8; PROC_NAME_LEN], 0),
    };
    Holder { pid: a.pid, name, name_len, measurement: *measurement, authority: a.authority }
}
