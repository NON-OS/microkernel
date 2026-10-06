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

//! The interrupt IN packet size a binding can work with.

use crate::protocol::{KEY_REPORT_LEN, MOUSE_REPORT_MIN, TABLET_REPORT_MIN};

use super::types::HidKind;

/// The most an interrupt endpoint moves in one packet at any USB speed.
pub const MAX_INTERRUPT_PACKET: u16 = 1024;

/// Whether one packet of `max_packet_size` bytes holds a whole report of
/// `kind`, and is no larger than an interrupt endpoint can be. The driver
/// reads one packet as one report, so a smaller packet would cut every
/// report short.
pub fn holds_a_report(kind: HidKind, max_packet_size: u16) -> bool {
    let report = match kind {
        HidKind::Keyboard => KEY_REPORT_LEN,
        HidKind::Mouse => MOUSE_REPORT_MIN,
        HidKind::Tablet => TABLET_REPORT_MIN,
    };
    (report as u16..=MAX_INTERRUPT_PACKET).contains(&max_packet_size)
}
