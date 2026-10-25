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

//! One HMAT structure: type u16, two reserved bytes, length u32, then the
//! kind's fields. Attributes: flags u16 (bit 0, the initiator is valid),
//! two reserved bytes, the initiator domain u32, the memory domain u32.
//! Locality: flags u8 (bits 3:0 the memory hierarchy), the data type u8,
//! two bytes, the initiator count u32, the target count u32, four reserved
//! bytes, the entry base unit u64, then the initiator domains, the target
//! domains and initiators × targets entries u16. Recalled: unverified.

use super::hmat_entry::{Attributes, Matrix};
use alloc::vec::Vec;

/// Domains on either side of one matrix.
const MAX_DOMAINS: usize = 256;

fn u32_at(s: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([s[at], s[at + 1], s[at + 2], s[at + 3]])
}

pub(super) fn attributes(s: &[u8]) -> Option<Attributes> {
    let s = s.get(..20)?;
    let valid = u16::from_le_bytes([s[8], s[9]]) & 1 != 0;
    let initiator = valid.then(|| u32_at(s, 12));
    Some(Attributes { memory: u32_at(s, 16), initiator })
}

pub(super) fn matrix(s: &[u8]) -> Option<Matrix> {
    let h = s.get(..32)?;
    let (m, n) = (u32_at(h, 12) as usize, u32_at(h, 16) as usize);
    if m > MAX_DOMAINS || n > MAX_DOMAINS || h[9] > 5 {
        return None;
    }
    let domains = |at: usize, k: usize| -> Option<Vec<u32>> {
        let b = s.get(at..at + 4 * k)?;
        Some(b.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect())
    };
    let initiators = domains(32, m)?;
    let targets = domains(32 + 4 * m, n)?;
    let e = s.get(32 + 4 * (m + n)..32 + 4 * (m + n) + 2 * m * n)?;
    let entries = e.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    let base_unit = u64::from(u32_at(h, 24)) | u64::from(u32_at(h, 28)) << 32;
    let (hierarchy, data_type) = (h[8] & 0x0F, h[9]);
    Some(Matrix { hierarchy, data_type, base_unit, initiators, targets, entries })
}
