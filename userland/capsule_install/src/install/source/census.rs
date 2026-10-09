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

/*
 * The capsules running now, counted by whose proof admitted them, from the
 * attestation registry. Each entry is pid, measurement, capability mask and
 * an authority byte: 0 the vendor's tree, 1 to 254 a root the user enrolled
 * on this machine, 255 a publisher's signature with no proof behind it.
 * Reading it needs AttestRead, which this capsule holds for this screen.
 */

use alloc::vec;

use nonos_libc::{mk_attest_entries, ATTEST_ENTRY_LEN};

/// The kernel's registry holds at most this many capsules.
const MAX_ENTRIES: usize = 256;
const AUTHORITY_AT: usize = ATTEST_ENTRY_LEN - 1;
const PUBLISHER: u8 = 255;

/// Running capsules by authority. Vendor and local ones passed a STARK check
/// at spawn; publisher ones were admitted on a signature alone.
#[derive(Clone, Copy)]
pub struct Census {
    pub vendor: u32,
    pub local: u32,
    pub publisher: u32,
}

/// `None` when the kernel refused the read or answered a length that is not
/// a whole number of entries.
pub fn census() -> Option<Census> {
    let mut buf = vec![0u8; MAX_ENTRIES * ATTEST_ENTRY_LEN];
    let rc = mk_attest_entries(&mut buf);
    if rc < 0 || rc as usize > buf.len() || !(rc as usize).is_multiple_of(ATTEST_ENTRY_LEN) {
        return None;
    }
    let mut c = Census { vendor: 0, local: 0, publisher: 0 };
    for entry in buf[..rc as usize].chunks_exact(ATTEST_ENTRY_LEN) {
        match entry[AUTHORITY_AT] {
            0 => c.vendor += 1,
            PUBLISHER => c.publisher += 1,
            _ => c.local += 1,
        }
    }
    Some(c)
}
