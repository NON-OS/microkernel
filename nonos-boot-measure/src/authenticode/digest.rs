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

use sha2::{Digest, Sha256};

use super::error::PeError;
use super::field::u32_at;
use super::pe::{layout, MAX_SECTIONS};

fn range(f: &[u8], from: usize, to: usize) -> Result<&[u8], PeError> {
    f.get(from..to).ok_or(PeError::OutOfFile)
}

/*
 * As the PE specification and EDK2's Tcg2MeasurePeImage compute it: the
 * headers without the checksum and the certificate table's directory entry,
 * then every section's raw data in file order, then whatever follows the
 * sections up to the certificate table. The certificate table itself is never
 * hashed, which is why a signature does not change the digest.
 */
pub fn digest(f: &[u8]) -> Result<[u8; 32], PeError> {
    let l = layout(f)?;
    let mut h = Sha256::new();
    h.update(range(f, 0, l.checksum)?);
    match l.cert_entry {
        Some(e) => {
            h.update(range(f, l.checksum + 4, e)?);
            h.update(range(f, e + 8, l.headers)?);
        }
        None => h.update(range(f, l.checksum + 4, l.headers)?),
    }
    let mut raw = [(0usize, 0usize); MAX_SECTIONS];
    for (i, slot) in raw.iter_mut().enumerate().take(l.n_sections) {
        let s = l.sections + 40 * i;
        *slot = (u32_at(f, s + 20)? as usize, u32_at(f, s + 16)? as usize);
    }
    let raw = &mut raw[..l.n_sections];
    /* Stable, as EDK2's insertion sort is: equal offsets keep table order. */
    for i in 1..raw.len() {
        let mut j = i;
        while j > 0 && raw[j - 1].0 > raw[j].0 {
            raw.swap(j - 1, j);
            j -= 1;
        }
    }
    let mut sum = l.headers;
    for &(at, size) in raw.iter().filter(|r| r.1 > 0) {
        h.update(range(f, at, at.checked_add(size).ok_or(PeError::OutOfFile)?)?);
        sum = sum.checked_add(size).ok_or(PeError::OutOfFile)?;
    }
    if f.len() > sum {
        let end = f.len().checked_sub(l.cert_size).ok_or(PeError::OutOfFile)?;
        if end < sum {
            return Err(PeError::OutOfFile);
        }
        h.update(range(f, sum, end)?);
    }
    Ok(h.finalize().into())
}
