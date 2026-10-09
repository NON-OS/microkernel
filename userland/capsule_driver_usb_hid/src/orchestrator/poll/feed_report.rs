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

use crate::descriptors::HidKind;
use crate::orchestrator::enumerate::HidEndpoint;
use crate::state::State;

use super::constants::HID_REPORT_MAX;

pub(super) fn feed_report(
    state: &mut State,
    ep: &HidEndpoint,
    buf: &[u8; HID_REPORT_MAX],
    n: usize,
) {
    // The read is bounded by the buffer; a count past it is no report.
    let Some(report) = buf.get(..n) else { return };
    match ep.kind {
        HidKind::Keyboard => {
            state.keyboard.feed(report);
            state.key_reports = state.key_reports.wrapping_add(1);
        }
        HidKind::Mouse => {
            state.mouse.feed(report);
            state.mouse_reports = state.mouse_reports.wrapping_add(1);
        }
        HidKind::Tablet => {
            state.tablet.feed(report);
        }
    }
}
