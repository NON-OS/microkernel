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

//! How much one interrupt read asks for. Pure, so the bound is proven on
//! the host.

use super::constants::HID_REPORT_MAX;

/// One packet of an endpoint whose packets are `max_packet` bytes, but no
/// more than a boot report. The controller driver refuses a longer read
/// with E_INVAL, so asking for a whole larger packet would leave the
/// endpoint unread for as long as it is bound.
pub fn read_len(max_packet: u16) -> u16 {
    max_packet.min(HID_REPORT_MAX as u16)
}
