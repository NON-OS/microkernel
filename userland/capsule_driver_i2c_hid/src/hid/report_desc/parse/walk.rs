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

use super::find_touch_report_id::find_touch_report_id;
use super::item::{next, TYPE_GLOBAL, TYPE_LOCAL, TYPE_MAIN};
use super::touch_walk::TouchWalk;
use super::usage_for::MAX_USAGES;
use crate::hid::report_desc::layout::TouchLayout;

pub fn parse(desc: &[u8]) -> TouchLayout {
    // A precision touchpad's descriptor declares the vestigial mouse
    // collection first (relative X/Y, no tip switch) and the absolute touch
    // collection after it. Only the touch collection carries a Digitizer Tip
    // Switch, so find that report id first and take every input field from it
    // alone; otherwise the parser locks onto the mouse collection's X and the
    // whole touch layout collapses to the relative boot-mouse path: random,
    // fast, click-spraying motion. Zero means the device uses no report ids
    // (single report), in which case every field is in that one report.
    let touch_rid = find_touch_report_id(desc);
    parse_with_rid(desc, touch_rid)
}

fn parse_with_rid(desc: &[u8], touch_rid: u8) -> TouchLayout {
    let mut layout = TouchLayout::default();
    let mut w = TouchWalk::default();
    let mut i = 0;
    while let Some(item) = next(desc, &mut i) {
        let data = item.data;
        match (item.btype, item.tag) {
            (TYPE_GLOBAL, 0x0) => w.usage_page = data as u16,
            (TYPE_GLOBAL, 0x2) => w.logical_max = data as i32,
            (TYPE_GLOBAL, 0x7) => w.report_size = data,
            (TYPE_GLOBAL, 0x8) => {
                w.report_id = data as u8;
                w.bit_offset = 0;
                w.feature_bit_offset = 0;
            }
            (TYPE_GLOBAL, 0x9) => w.report_count = data,
            // Local Usage; ignore usage min/max ranges for this purpose.
            (TYPE_LOCAL, 0x0) if w.usages.n < MAX_USAGES => {
                w.usages.list[w.usages.n] = data as u16;
                w.usages.n += 1;
            }
            (TYPE_MAIN, tag) => w.main_item(&mut layout, tag, touch_rid),
            _ => {}
        }
    }
    layout
}
