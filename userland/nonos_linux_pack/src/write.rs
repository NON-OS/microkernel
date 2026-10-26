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

//! The bytes of a pack laid out by build.rs: the header, each entry with
//! its digest, the names and the data, every gap zero.

use alloc::vec::Vec;

use crate::build::Source;
use crate::format::{DIGEST_LEN, ENTRY_LEN, HEADER_LEN, MAGIC, VERSION};
use crate::kind::{bytes as kind_bytes, Kind};
use crate::refused::Refused;

/// `at`, each source's data offset; `names`, (names_off, names_len,
/// data_off); `end`, the pack's length. Memory, when the pack's length is
/// more than is free: a package too large to pack is refused, never a panic.
pub fn write(
    sources: &[Source],
    at: &[usize],
    names: (usize, usize, usize),
    end: usize,
) -> Result<Vec<u8>, Refused> {
    let (names_off, names_len, data_off) = names;
    let mut out = Vec::new();
    out.try_reserve_exact(end).map_err(|_| Refused::Memory)?;
    out.resize(end, 0);
    let words = [names_off as u64, names_len as u64, data_off as u64];
    out[..8].copy_from_slice(&MAGIC);
    out[8..10].copy_from_slice(&VERSION.to_le_bytes());
    out[12..16].copy_from_slice(&(sources.len() as u32).to_le_bytes());
    out[16..24].copy_from_slice(&(HEADER_LEN as u64).to_le_bytes());
    out[24..32].copy_from_slice(&((sources.len() * ENTRY_LEN) as u64).to_le_bytes());
    for (i, w) in words.iter().enumerate() {
        out[32 + i * 8..40 + i * 8].copy_from_slice(&w.to_le_bytes());
    }
    let mut name_at = names_off;
    for (i, (s, &start)) in sources.iter().zip(at).enumerate() {
        let e = &mut out[HEADER_LEN + i * ENTRY_LEN..][..ENTRY_LEN];
        e[0..4].copy_from_slice(&(name_at as u32).to_le_bytes());
        e[4..6].copy_from_slice(&(s.name.len() as u16).to_le_bytes());
        e[6..8].copy_from_slice(&kind_bytes(s.kind, s.exec));
        e[8..16].copy_from_slice(&(start as u64).to_le_bytes());
        e[16..24].copy_from_slice(&(s.bytes.len() as u64).to_le_bytes());
        if s.kind != Kind::Dir {
            e[24..24 + DIGEST_LEN]
                .copy_from_slice(&blake3::hash(&s.bytes).as_bytes()[..DIGEST_LEN]);
        }
        out[name_at..name_at + s.name.len()].copy_from_slice(&s.name);
        out[start..start + s.bytes.len()].copy_from_slice(s.bytes);
        name_at += s.name.len();
    }
    Ok(out)
}
