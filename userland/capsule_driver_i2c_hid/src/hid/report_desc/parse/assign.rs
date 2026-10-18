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

//! An input field of the touch report, recorded the first time its usage
//! is seen, so a report that repeats X and Y for every finger keeps the
//! first contact's. Usages are page in the top half, usage in the bottom.

use super::field_at::FieldAt;
use crate::hid::report_desc::{Field, TouchLayout};

const X: u32 = 0x0001_0030;
const Y: u32 = 0x0001_0031;
pub(super) const TIP_SWITCH: u32 = 0x000D_0042;
const CONFIDENCE: u32 = 0x000D_0047;
const CONTACT_ID: u32 = 0x000D_0051;
const CONTACT_COUNT: u32 = 0x000D_0054;
const BUTTON_PAGE: u32 = 0x0009;

pub(super) fn assign(l: &mut TouchLayout, f: &FieldAt) {
    let slot: &mut Field = match f.usage {
        X if !f.relative => &mut l.x,
        Y if !f.relative => &mut l.y,
        TIP_SWITCH => &mut l.tip,
        CONFIDENCE => &mut l.confidence,
        CONTACT_ID => &mut l.contact_id,
        CONTACT_COUNT => &mut l.contact_count,
        u if u >> 16 == BUTTON_PAGE => &mut l.button,
        _ => return,
    };
    if !slot.present() {
        *slot = f.field();
    }
}
