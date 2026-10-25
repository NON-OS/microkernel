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

//! What version 5 adds after the DMA posture: the quoted PCRs, each index
//! and its SHA-256 value, exactly the kernel's eight in its order, then the
//! firmware's event log, length first.

const QUOTED: [u8; 8] = [0, 1, 2, 4, 7, 8, 9, 14];
const MAX_LOG: u32 = 1024 * 1024;

/// Where the attest length sits after them, and how long the log is.
pub(super) fn pcrs_and_log(b: &[u8], at: usize) -> Option<(usize, u32)> {
    if usize::from(*b.get(at)?) != QUOTED.len() {
        return None;
    }
    for (k, want) in QUOTED.iter().enumerate() {
        if b.get(at + 1 + 33 * k)? != want {
            return None;
        }
    }
    let log_at = at + 1 + 33 * QUOTED.len();
    let log_len = be32(b, log_at)?;
    if log_len > MAX_LOG {
        return None;
    }
    Some((log_at.checked_add(4)?.checked_add(log_len as usize)?, log_len))
}

pub(super) fn be32(b: &[u8], at: usize) -> Option<u32> {
    let s = b.get(at..at + 4)?;
    Some(u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
}
