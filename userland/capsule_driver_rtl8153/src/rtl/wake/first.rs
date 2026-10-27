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

//! The first RTL8153's wake (codes 5c00 to 5c30).

use alloc::vec::Vec;

use crate::rtl::bounds::AUTOLOAD;
use crate::rtl::common::{no_u1u2, NO_D3, NO_SUSPEND_WAKE, NO_U2P3, ONE_FRAME_IN, PHY_UP};
use crate::rtl::model::Model;
use crate::rtl::op::{clear, put, set, until, Op, Target};
use crate::rtl::reg::{pla16, pla8, usb16, usb8};
use crate::rtl::regs::{phy, pla, usb};
use crate::rtl::usb_speed::UsbSpeed;

/// The 5c10 chip's SuperSpeed power-down scale (96) and USB 2 L1 and
/// suspend; the 5c20's ALDPS in ECM mode off; the 5c30's D3 flags clear.
const AT_5C10: [Op; 2] =
    [Op::Edit(Target::Mac(usb16(usb::SS_LINK2)), 0x3ffe, 96 << 1), set(usb8(usb::USB2_PHY), 0x03)];
const AT_5C20: [Op; 1] = [clear(pla8(pla::SPARE0), 0x02)];

pub(super) fn wake(m: Model, code: u16, speed: UsbSpeed) -> Vec<Op> {
    let mut v = Vec::with_capacity(40);
    v.extend_from_slice(no_u1u2(m));
    v.push(until(pla16(pla::BOOT), 0x0002, &[0x0002], AUTOLOAD));
    v.push(PHY_UP[0]);
    if matches!(code, 0x5c00 | 0x5c10 | 0x5c20) {
        v.push(Op::Put(Target::Phy(phy::ADC_CFG), 0x0100 | 0x0080 | 0x0040));
    }
    v.extend_from_slice(&PHY_UP[1..]);
    v.push(NO_U2P3);
    v.extend_from_slice(match code {
        0x5c10 => &AT_5C10[..],
        0x5c20 => &AT_5C20[..],
        0x5c30 => &NO_D3[..],
        _ => &[],
    });
    // The low-power timer: 500 ms for the 5c10 below SuperSpeed, else 500 us.
    let lpm = if code == 0x5c10 && speed != UsbSpeed::Super { 0x04 } else { 0x0c };
    v.extend_from_slice(&[
        set(usb8(usb::SPARE2), 0x01), // flow control when endpoint 4 is full
        clear(usb16(usb::WATCHDOG11), 0x0001),
        clear(pla16(pla::LEDS), 0x0700),
        put(usb8(usb::LPM_CTRL), 0x30 | 0x02 | lpm),
        Op::Edit(Target::Mac(usb16(usb::AFE2)), 0xf800, 0xa000 | 0x0100),
        put(usb16(usb::CONNECT_TIMER), 0x0001),
        clear(usb16(usb::POWER_CUT), 0x0001 | 0x0008),
        clear(usb16(usb::MISC0), 0x0001),
    ]);
    v.extend_from_slice(&NO_SUSPEND_WAKE);
    v.extend_from_slice(&[
        clear(pla16(pla::MAC_POWER2), 0x8000), // MAC clock not slowed
        set(pla8(pla::CONFIG6), 0x01),
        clear(pla8(pla::LAN_WAKE), 0x80),
        ONE_FRAME_IN,
        set(pla16(pla::TALLY), 0x0001),
    ]);
    v
}
