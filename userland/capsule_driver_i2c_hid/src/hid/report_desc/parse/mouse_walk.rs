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

//! Every input field of a report descriptor, in order, with the report id,
//! usage page, usage, bit offset, size and Input flags it is declared with.
//! Usage Minimum/Maximum ranges are expanded, as button arrays use them.

use super::item::next;
use super::mouse_locals::Locals;

const MAX_FIELDS_PER_ITEM: u32 = 256;

/// One input field as the walk meets it.
pub(super) struct InputField {
    pub report_id: u8,
    pub page: u16,
    pub usage: u16,
    pub bit_offset: u32,
    pub bit_size: u32,
    pub flags: u32,
}

pub(super) fn walk_inputs(desc: &[u8], mut f: impl FnMut(&InputField)) {
    let (mut page, mut size, mut count, mut rid) = (0u16, 0u32, 0u32, 0u8);
    let mut offset = 0u32;
    let mut locals = Locals::default();
    let mut i = 0usize;
    while let Some(item) = next(desc, &mut i) {
        let (data, tag) = (item.data, item.tag);
        match item.btype {
            1 => match tag {
                0x0 => page = data as u16,
                0x7 => size = data,
                0x8 => {
                    rid = data as u8;
                    offset = 0;
                }
                0x9 => count = data,
                _ => {}
            },
            2 => locals.local(tag, data),
            0 => {
                if tag == 0x8 {
                    for k in 0..count.min(MAX_FIELDS_PER_ITEM) {
                        f(&InputField {
                            report_id: rid,
                            page,
                            usage: locals.usage(k),
                            bit_offset: offset,
                            bit_size: size,
                            flags: data,
                        });
                        offset = offset.saturating_add(size);
                    }
                }
                locals.clear();
            }
            _ => {}
        }
    }
}
