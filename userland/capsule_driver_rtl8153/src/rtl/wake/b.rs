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

//! The RTL8153B's wake (codes 6000 and 6010). Linux v6.12 also slows the
//! MAC clock here, a power saving this driver leaves off, and seeds the
//! link-change poll from the link, which this driver reads itself.

use alloc::vec::Vec;

use crate::rtl::bounds::AUTOLOAD;
use crate::rtl::common::{no_u1u2, NO_D3, NO_SUSPEND_WAKE, NO_U2P3, ONE_FRAME_IN, PHY_UP};
use crate::rtl::model::Model;
use crate::rtl::op::{clear, put, set, until, Op, Target};
use crate::rtl::reg::{pla16, pla8, usb16, usb8};
use crate::rtl::regs::{pla, usb};

pub(super) fn wake(m: Model, code: u16) -> Vec<Op> {
    let mut v = Vec::with_capacity(32);
    v.extend_from_slice(no_u1u2(m));
    v.push(until(pla16(pla::BOOT), 0x0002, &[0x0002], AUTOLOAD));
    v.extend_from_slice(&PHY_UP);
    v.extend_from_slice(&[
        NO_U2P3,
        put(usb16(usb::MSC_TIMER), 0x0fff),
        clear(usb16(usb::POWER_CUT), 0x0001),
        clear(usb16(usb::MISC0), 0x0001),
        // UPS, the chip's own power-down when idle, off and not forced.
        clear(usb8(usb::POWER_CUT), 0x10 | 0x20),
        clear(usb8(usb::MISC2), 0x01),
    ]);
    v.extend_from_slice(&NO_D3);
    v.extend_from_slice(&NO_SUSPEND_WAKE);
    v.push(clear(pla16(pla::MAC_POWER3), 0x4000)); // MCU clock not slowed
    if code == 0x6010 {
        // A 32-pin package has its test I/O turned off.
        let pkg = Target::Mac(pla8(pla::PACKAGE));
        v.push(Op::EditIf(pkg, 0x20, Target::Mac(pla16(pla::PHY_POWER)), 0, 0x10));
    }
    v.extend_from_slice(&[ONE_FRAME_IN, set(pla16(pla::TALLY), 0x0001)]);
    v
}
