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

//! An LPC bridge's legacy devices as firmware writes them, EISA ids and
//! all: keyboard and PS/2 mouse (by _CID), a UART, the RTC, an HPET whose
//! _STA is computed, and a PIT that says absent.

use super::build::{crs, fixed_io, io, irq, mem32};
use crate::enc::*;

fn dev(path: &str, hid: u64, rest: &[u8]) -> Vec<u8> {
    device(path, &cat(&[&name_obj("_HID", &int(hid)), rest]))
}

pub fn machine(with_ps2: bool) -> Vec<u8> {
    let kbd = dev("\\_SB.LPCB.KBD0", 0x0303_D041, &crs(&[io(0x60, 1), io(0x64, 1), irq(1)]));
    let mouse = device(
        "\\_SB.LPCB.PS2M",
        &cat(&[
            &name_obj("_HID", &string("SYN0A00")),
            &name_obj("_CID", &int(0x130F_D041)),
            &crs(&[irq(12)]),
        ]),
    );
    let hpet_sta = method("_STA", 0, &ret(&int(0x0F)));
    let ps2 = if with_ps2 { cat(&[&kbd, &mouse]) } else { Vec::new() };
    cat(&[
        &device("\\_SB.LPCB", &[]),
        &ps2,
        &dev(
            "\\_SB.LPCB.UAR1",
            0x0105_D041,
            &cat(&[&name_obj("_UID", &int(1)), &crs(&[io(0x3F8, 8), irq(4)])]),
        ),
        &dev("\\_SB.LPCB.RTC0", 0x000B_D041, &crs(&[fixed_io(0x70, 2)])),
        &dev(
            "\\_SB.LPCB.HPET",
            0x0301_D041,
            &cat(&[&hpet_sta, &crs(&[mem32(0xFED0_0000, 0x400)])]),
        ),
        &dev(
            "\\_SB.LPCB.TIMR",
            0x0001_D041,
            &cat(&[&name_obj("_STA", &int(0)), &crs(&[io(0x40, 4)])]),
        ),
    ])
}
