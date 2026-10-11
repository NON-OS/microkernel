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

//! The ADMA2 table walked as the host walks it: the data segments of a
//! transfer, or None on an ADMA error, with the rules a table must keep.

use super::sim::Sim;
use super::types::Desc;

impl Sim {
    pub(super) fn walk(&mut self, len: usize) -> Option<Vec<(u64, usize)>> {
        let (mut at, wide) = self.table_start()?;
        let dl = if wide { 12 } else { 8 };
        let mut segs = Vec::new();
        let mut table = Vec::new();
        let mut total = 0usize;
        for _ in 0..64 {
            let Desc { attr, len: l, addr } = self.desc(at, wide);
            table.push(Desc { attr, len: l, addr });
            if attr & 1 == 0 {
                self.tables.push(table);
                return None;
            }
            let act = (attr >> 4) & 3;
            let end = attr & 2 != 0;
            match act {
                2 => {
                    let l = if l == 0 { 65536 } else { l as usize };
                    if addr % 4 != 0 {
                        self.violate(format!("data address {addr:#x} misaligned"));
                    }
                    if !wide && addr + l as u64 > 1 << 32 {
                        self.violate("32-bit descriptor past 4 GiB".into());
                    }
                    segs.push((addr, l));
                    total += l;
                }
                0 if end => self.violate("Nop descriptor with End".into()),
                0 => {}
                3 => {
                    at = addr;
                    continue;
                }
                _ => self.violate("reserved Act".into()),
            }
            if end {
                self.tables.push(table);
                if total != len {
                    self.violate(format!("descriptors move {total} bytes, command moves {len}"));
                    return None;
                }
                return Some(segs);
            }
            at += dl as u64;
        }
        self.violate("descriptor table without End".into());
        self.tables.push(table);
        None
    }
}
