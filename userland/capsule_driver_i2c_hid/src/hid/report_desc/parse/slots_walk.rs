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
//! Every finger's fields in the touch report (multitouch/slots.rs): the
//! same walk as the touch layout's, each Input field of report `rid` given
//! to the slots in the order the descriptor declares it.

use super::item::{next, TYPE_GLOBAL, TYPE_LOCAL, TYPE_MAIN};
use super::usage_for::{usage_for, Usages, MAX_USAGES};
use crate::hid::report_desc::layout::Field;
use crate::hid::report_desc::multitouch::Slots;

const MAX_FIELDS_PER_ITEM: u32 = 256;

pub fn parse_slots(desc: &[u8], rid: u8) -> Slots {
    let (mut slots, mut usages) = (Slots::default(), Usages::default());
    let (mut page, mut size, mut count, mut max, mut id, mut at) =
        (0u16, 0u32, 0u32, 0i32, 0u8, 0u32);
    let mut i = 0;
    while let Some(item) = next(desc, &mut i) {
        let data = item.data;
        match (item.btype, item.tag) {
            (TYPE_GLOBAL, 0x0) => page = data as u16,
            (TYPE_GLOBAL, 0x2) => max = data as i32,
            (TYPE_GLOBAL, 0x7) => size = data,
            (TYPE_GLOBAL, 0x8) => (id, at) = (data as u8, 0),
            (TYPE_GLOBAL, 0x9) => count = data,
            (TYPE_LOCAL, 0x0) if usages.n < MAX_USAGES => {
                usages.list[usages.n] = data as u16;
                usages.n += 1;
            }
            (TYPE_MAIN, tag) => {
                for f in 0..count.min(MAX_FIELDS_PER_ITEM) {
                    if tag == 0x8 && id == rid {
                        let usage = usage_for(&usages.list, usages.n, f as usize);
                        if let Some(field) = Field::bounded(at, size, max) {
                            slots.record(page, usage, field);
                        }
                    }
                    if tag == 0x8 {
                        at = at.saturating_add(size);
                    }
                }
                usages.n = 0;
            }
            _ => {}
        }
    }
    slots
}
