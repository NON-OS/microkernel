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

//! Splitting a CERTS cell into its certificates.

/// One certificate as the cell carries it.
pub struct Entry<'a> {
    pub cert_type: u8,
    pub body: &'a [u8],
}

/// Walk a CERTS body: N_CERTS then, per certificate, CertType and a two byte
/// length. `None` if a stated length runs past the cell.
pub fn split(body: &[u8]) -> Option<impl Iterator<Item = Entry<'_>>> {
    let count = *body.first()? as usize;
    let mut rest = body.get(1..)?;
    let mut out = 0usize;
    /*
     * Walked twice: once to refuse a malformed cell outright, then again to
     * hand entries out. A cell whose last length overruns is not one to read
     * the earlier certificates from.
     */
    let mut probe = rest;
    for _ in 0..count {
        if probe.len() < 3 {
            return None;
        }
        let len = u16::from_be_bytes([probe[1], probe[2]]) as usize;
        probe = probe.get(3 + len..)?;
        out += 1;
    }
    Some(core::iter::from_fn(move || {
        if out == 0 {
            return None;
        }
        out -= 1;
        let len = u16::from_be_bytes([rest[1], rest[2]]) as usize;
        let entry = Entry { cert_type: rest[0], body: &rest[3..3 + len] };
        rest = &rest[3 + len..];
        Some(entry)
    }))
}
