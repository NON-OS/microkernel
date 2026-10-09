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
//! recorded with its own report id the first time it is seen.

use crate::hid::report_desc::layout::{Field, TouchLayout};

const USAGE_PAGE_DIGITIZER: u16 = 0x0D;
const USAGE_INPUT_MODE: u16 = 0x52;
const USAGE_SURFACE_SWITCH: u16 = 0x57;
const USAGE_BUTTON_SWITCH: u16 = 0x58;

pub(super) fn feature(layout: &mut TouchLayout, usage_page: u16, usage: u16, rid: u8, f: Field) {
    if usage_page != USAGE_PAGE_DIGITIZER {
        return;
    }
    match usage {
        USAGE_INPUT_MODE if !layout.input_mode.present() => {
            layout.input_mode = f;
            layout.input_mode_report_id = rid;
        }
        USAGE_SURFACE_SWITCH if !layout.surface_switch.present() => {
            layout.surface_switch = f;
            layout.surface_switch_report_id = rid;
        }
        USAGE_BUTTON_SWITCH if !layout.button_switch.present() => {
            layout.button_switch = f;
            layout.button_switch_report_id = rid;
        }
        _ => {}
    }
}
