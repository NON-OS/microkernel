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
//! One Input item laid out: `count` fields of `size` bits each, after
//! those before them in the same report. Constant (padding) and array
//! items take their bits and give no field; variable fields of the usages
//! this driver reads are kept while inside a gamepad application.

use alloc::vec::Vec;

use super::layout::{wanted, Field, PadLayout};
use super::usages::Usages;

#[derive(Clone, Copy, Default)]
pub struct Globals {
    pub page: u16,
    pub min: i32,
    pub max: i32,
    pub size: u32,
    pub count: u32,
    pub id: u8,
}

/// Fields kept, at most; a descriptor that lists more gives no more.
const FIELDS: usize = 64;

pub fn input(
    g: &Globals,
    u: &Usages,
    data: u32,
    pad: bool,
    offsets: &mut Vec<(u8, u32)>,
    layout: &mut PadLayout,
) {
    let at = match offsets.iter().position(|o| o.0 == g.id) {
        Some(i) => i,
        None => {
            offsets.push((g.id, 0));
            offsets.len() - 1
        }
    };
    let start = offsets[at].1;
    offsets[at].1 = start.saturating_add(g.size.saturating_mul(g.count));
    let variable = data & 0x01 == 0 && data & 0x02 != 0;
    if !pad || !variable || !(1..=32).contains(&g.size) {
        return;
    }
    for i in 0..g.count.min(FIELDS as u32) {
        let Some((page, usage)) = u.of(i) else { break };
        if wanted(page, usage) && layout.fields.len() < FIELDS {
            let bit_offset = start.saturating_add(i * g.size);
            let (min, max, report_id, bit_size) = (g.min, g.max, g.id, g.size);
            layout.fields.push(Field { page, usage, report_id, bit_offset, bit_size, min, max });
        }
    }
}
