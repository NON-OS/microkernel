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

//! The three target fields, and each feature report that holds one of them
//! configured once.

use super::one_report::configure_one_report;
use crate::hid::report_desc::TouchLayout;

/// The Precision Touchpad collection. Modern PTP-only pads implement the
/// mouse collection for compliance only, and it never generates reports;
/// Windows and Linux both switch such pads to mode 3 on bind, so touch
/// mode is the only mode this device class actually reports in. Read
/// pacing comes from the GPIO doorbell.
const INPUT_MODE_TOUCHPAD: u32 = 3;
const SWITCH_ON: u32 = 1;

/// Drive input mode to touchpad and both reporting switches on, touching only
/// feature reports that need a change. Returns true when the configuration
/// is known good (already correct, or corrected).
pub fn configure_reporting(port: u32, addr: u8, desc: &[u8; 30], layout: &TouchLayout) -> bool {
    let targets = [
        (layout.input_mode, layout.input_mode_report_id, INPUT_MODE_TOUCHPAD),
        (layout.surface_switch, layout.surface_switch_report_id, SWITCH_ON),
        (layout.button_switch, layout.button_switch_report_id, SWITCH_ON),
    ];
    let mut all_ok = true;
    let mut done_ids = [0u8; 3];
    let mut done = 0usize;
    for (_, id, _) in targets.iter() {
        let id = *id;
        // Id 0 means the field is absent.
        if id == 0 || done_ids[..done].contains(&id) {
            continue;
        }
        done_ids[done] = id;
        done += 1;
        all_ok &= configure_one_report(port, addr, desc, id, &targets);
    }
    all_ok
}
