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

use crate::controller::ControllerInfo;
use crate::log::Line;

/// The HBA's capabilities, the ports it implements and the ones walked, and
/// how much of ABAR is mapped: one line per controller opened.
pub(super) fn say_hba(info: &ControllerInfo, raw_pi: u32, mapped: u64) {
    Line::new()
        .text(b"CAP ")
        .hex(info.cap)
        .text(b" CAP2 ")
        .hex(info.cap2)
        .text(b" VS ")
        .hex(info.version)
        .text(b" PI ")
        .hex(raw_pi)
        .text(b" walked ")
        .hex(info.pi)
        .text(b" ABAR mapped ")
        .num(mapped)
        .send();
}
