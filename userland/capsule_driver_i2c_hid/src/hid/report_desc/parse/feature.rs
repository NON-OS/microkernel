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

//! The device-configuration switches a precision touchpad declares in its
//! Feature reports: Input Mode, Surface Switch and Button Switch, each
//! recorded with its own report ID the first time it is seen, and placed
//! in the bits of that feature report alone.

use super::field_at::FieldAt;
use crate::hid::report_desc::TouchLayout;

const INPUT_MODE: u32 = 0x000D_0052;
const SURFACE_SWITCH: u32 = 0x000D_0057;
const BUTTON_SWITCH: u32 = 0x000D_0058;

pub(super) fn feature(l: &mut TouchLayout, f: &FieldAt) {
    let (field, id) = match f.usage {
        INPUT_MODE => (&mut l.input_mode, &mut l.input_mode_report_id),
        SURFACE_SWITCH => (&mut l.surface_switch, &mut l.surface_switch_report_id),
        BUTTON_SWITCH => (&mut l.button_switch, &mut l.button_switch_report_id),
        _ => return,
    };
    if !field.present() {
        *field = f.field();
        *id = f.id;
    }
}
