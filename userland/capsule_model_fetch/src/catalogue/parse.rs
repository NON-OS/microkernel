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
 * The catalogue's layout, as tools/nonos_qwen_tier/wire.py writes it:
 *
 *   "NXQWEN01" | serial u64 | NONOS mirror base: u16 length, bytes |
 *   tiers u16 | each tier: word (u8 length), memory u64, files u8 |
 *   each file: name (u8 length), length u64, SHA-256 (32), mirrors u8,
 *   each mirror: u16 length, URL
 *
 * little-endian, then an Ed25519 signature over all of it.
 */

use alloc::vec::Vec;

use super::read::Reader;
use super::types::{Catalogue, File, Tier};

const MAGIC: &[u8; 8] = b"NXQWEN01";
const URL_MAX: usize = 2048;

pub fn parse(body: &[u8]) -> Option<Catalogue> {
    let mut r = Reader::new(body);
    if r.take(8)? != MAGIC {
        return None;
    }
    let (serial, base) = (r.u64()?, r.text(true, URL_MAX)?);
    let mut tiers = Vec::new();
    for _ in 0..r.u16()? {
        let (word, memory) = (r.text(false, 32)?, r.u64()?);
        let mut files = Vec::new();
        for _ in 0..r.u8()? {
            let (name, bytes) = (r.text(false, 63)?, r.u64()?);
            let sha256: [u8; 32] = r.take(32)?.try_into().ok()?;
            let count = r.u8()?;
            let mirrors = (0..count).map(|_| r.text(true, URL_MAX)).collect::<Option<Vec<_>>>()?;
            files.push(File { name, bytes, sha256, mirrors });
        }
        tiers.push(Tier { word, memory, files });
    }
    r.is_empty().then_some(Catalogue { serial, base, tiers })
}
