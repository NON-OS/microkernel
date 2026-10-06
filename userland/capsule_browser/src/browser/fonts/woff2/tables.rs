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

use alloc::vec::Vec;

use super::directory::{find, Table};
use super::{glyf, hmtx};

/// Every table's sfnt bytes, in directory order, from the decompressed
/// stream `raw`: untransformed tables as they are, glyf and loca rebuilt
/// together, and hmtx rebuilt from the glyphs' xMin values.
pub(super) fn rebuild(tables: &[Table], raw: &[u8]) -> Option<Vec<([u8; 4], Vec<u8>)>> {
    let head = &raw[find(tables, b"head")?.src.clone()];
    let long_loca = head.get(50..52)? != [0, 0];
    let mut outlines = None;
    let (glyf_t, loca_t) = (find(tables, b"glyf"), find(tables, b"loca"));
    if glyf_t.is_some() != loca_t.is_some() {
        return None;
    }
    if let (Some(g), Some(l)) = (glyf_t, loca_t) {
        if g.transformed != l.transformed {
            return None;
        }
        if g.transformed {
            outlines = Some(glyf::rebuild(&raw[g.src.clone()], l.orig_len, long_loca)?);
        }
    }
    let mut out: Vec<([u8; 4], Vec<u8>)> = Vec::new();
    out.try_reserve_exact(tables.len()).ok()?;
    for t in tables {
        let bytes = match (&t.tag, t.transformed, &outlines) {
            (_, false, _) => raw[t.src.clone()].to_vec(),
            (b"glyf", true, Some(o)) => o.glyf.clone(),
            (b"loca", true, Some(o)) => o.loca.clone(),
            (b"hmtx", true, Some(o)) => {
                let hhea = &raw[find(tables, b"hhea")?.src.clone()];
                let metrics = u16::from_be_bytes([*hhea.get(34)?, *hhea.get(35)?]) as usize;
                hmtx::rebuild(&raw[t.src.clone()], metrics, &o.x_mins)?
            }
            _ => return None,
        };
        out.push((t.tag, bytes));
    }
    Some(out)
}
