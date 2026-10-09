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

//! The Endpoint Context Interval field for an interrupt endpoint (xHCI 1.2
//! section 6.2.3.6, Linux `xhci_get_endpoint_interval`). The controller
//! wants an exponent: the endpoint is served every 2^Interval * 125 us. The
//! endpoint descriptor's bInterval means different things by speed: at full
//! and low speed it is a period in 1 ms frames (1 to 255), at high speed and
//! above an exponent of 125 us microframes plus one (1 to 16). Passing it
//! through unconverted serves a 10 ms keyboard every 128 ms, and a bInterval
//! of 16 or more at full speed is a reserved Interval the controller rejects
//! with a Parameter Error, so Configure Endpoint fails.

use super::ep0::{is_superspeed, SPEED_HIGH};

/// The shortest and longest full and low speed periods served: 1 ms and
/// 128 ms as exponents of 125 us, as Linux clamps them.
const FS_MIN_EXP: u32 = 3;
const FS_MAX_EXP: u32 = 10;

pub fn interrupt_interval(speed: u8, usb3: bool, b_interval: u8) -> u8 {
    if is_superspeed(speed, usb3) || speed == SPEED_HIGH {
        return b_interval.clamp(1, 16) - 1;
    }
    let microframes = b_interval.max(1) as u32 * 8;
    let exp = 31 - microframes.leading_zeros();
    exp.clamp(FS_MIN_EXP, FS_MAX_EXP) as u8
}
