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
use crate::contexts::{is_superspeed, SPEED_HIGH};

/// The bulk max packet size the controller is given. USB 2.0 fixes it at
/// 512 for high speed and USB 3 at 1024 for SuperSpeed, and some USB sticks
/// report another value; Linux's `xhci_endpoint_init` overrides it the same
/// way ("Some devices get this wrong"). Full speed keeps what the descriptor
/// says, bits 10:0.
pub(super) fn bulk_max_packet(speed: u8, usb3: bool, descriptor: u16) -> u16 {
    if is_superspeed(speed, usb3) {
        1024
    } else if speed == SPEED_HIGH {
        512
    } else {
        descriptor & 0x07FF
    }
}
