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

//! The state the touch layout walk carries between items, and what a main
//! item does with it: an Input item lays out input-report fields, a Feature
//! item is walked only for the configuration switches, and every main item
//! clears the locals.

use super::assign::assign;
use super::feature::feature;
use super::usage_for::{usage_for, Usages};
use crate::hid::report_desc::layout::{Field, TouchLayout};

const MAX_FIELDS_PER_ITEM: u32 = 256;

#[derive(Default)]
pub(super) struct TouchWalk {
    pub usage_page: u16,
    pub report_size: u32,
    pub report_count: u32,
    pub report_id: u8,
    pub logical_max: i32,
    pub bit_offset: u32,
    /// Feature reports lay out in their own offset space, separate from
    /// input reports of the same id.
    pub feature_bit_offset: u32,
    pub usages: Usages,
}

impl TouchWalk {
    pub fn main_item(&mut self, layout: &mut TouchLayout, tag: u8, touch_rid: u8) {
        let count = self.report_count.min(MAX_FIELDS_PER_ITEM);
        let (page, size, max, rid) =
            (self.usage_page, self.report_size, self.logical_max, self.report_id);
        if tag == 0x8 {
            for f in 0..count {
                // Input fields count only when they live in the touch report:
                // a mouse-collection X at the same usage would otherwise be
                // recorded and drag the whole layout onto the wrong report.
                if rid == touch_rid {
                    let usage = usage_for(&self.usages.list, self.usages.n, f as usize);
                    assign(layout, page, usage, rid, self.bit_offset, size, max);
                }
                self.bit_offset = self.bit_offset.saturating_add(size);
            }
        } else if tag == 0xB {
            for f in 0..count {
                let usage = usage_for(&self.usages.list, self.usages.n, f as usize);
                let field =
                    Field { bit_offset: self.feature_bit_offset, bit_size: size, logical_max: max };
                feature(layout, page, usage, rid, field);
                self.feature_bit_offset = self.feature_bit_offset.saturating_add(size);
            }
        }
        // Locals are cleared after every main item.
        self.usages.n = 0;
    }
}
