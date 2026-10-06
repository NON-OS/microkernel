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

use super::item::{next, TYPE_GLOBAL, TYPE_LOCAL, TYPE_MAIN};
use super::usage_for::{usage_for, Usages, MAX_USAGES};

const USAGE_PAGE_DIGITIZER: u16 = 0x0D;
const USAGE_TIP_SWITCH: u16 = 0x42;

const MAX_FIELDS_PER_ITEM: u32 = 256;

// The report id of the input report that declares a Digitizer Tip Switch.
// Zero when none is found (also the no-report-id case).
pub(super) fn find_touch_report_id(desc: &[u8]) -> u8 {
    let (mut usage_page, mut report_id, mut report_count) = (0u16, 0u8, 0u32);
    let mut usages = Usages::default();
    let mut i = 0;
    while let Some(item) = next(desc, &mut i) {
        let data = item.data;
        match (item.btype, item.tag) {
            (TYPE_GLOBAL, 0x0) => usage_page = data as u16,
            (TYPE_GLOBAL, 0x8) => report_id = data as u8,
            (TYPE_GLOBAL, 0x9) => report_count = data,
            (TYPE_LOCAL, 0x0) if usages.n < MAX_USAGES => {
                usages.list[usages.n] = data as u16;
                usages.n += 1;
            }
            (TYPE_MAIN, tag) => {
                if tag == 0x8 {
                    for f in 0..report_count.min(MAX_FIELDS_PER_ITEM) {
                        let usage = usage_for(&usages.list, usages.n, f as usize);
                        if usage_page == USAGE_PAGE_DIGITIZER && usage == USAGE_TIP_SWITCH {
                            return report_id;
                        }
                    }
                }
                usages.n = 0;
            }
            _ => {}
        }
    }
    0
}
