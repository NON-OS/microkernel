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

//! The per-version register map against Linux (r8169_main.c): the 8168
//! rings TxPoll (0x38, 8 bits, NPQ 0x40) and masks at 0x3C/0x3E (16 bits);
//! the 8125 rings TxPoll_8125 (0x90, 16 bits, bit 0) and keeps a 32-bit
//! IntrMask_8125 at 0x38 and IntrStatus_8125 at 0x3C.

use super::memory::Memory;
use super::model::window;
use crate::chip::{Chip, MacVersion};
use crate::regmap::layout::{Doorbell, EventRegs};
use crate::regmap::{doorbell_of, events_of, mask_and_ack};

pub(super) const RTL8125B: Chip = Chip::new(63, "RTL8125B", 0x641);

#[test]
fn each_version_gets_the_linux_doorbell_and_event_registers() {
    for v in [2u8, 10, 17, 25, 34, 40, 46, 52] {
        let (bell, ev) = (doorbell_of(MacVersion(v)), events_of(MacVersion(v)));
        assert_eq!(bell, Doorbell { offset: 0x38, value: 0x40, wide: false }, "VER_{v}");
        assert_eq!(ev, EventRegs { imr: 0x3C, isr: 0x3E, wide: false }, "VER_{v}");
    }
    for v in [61u8, 63, 64, 65, 66] {
        let (bell, ev) = (doorbell_of(MacVersion(v)), events_of(MacVersion(v)));
        assert_eq!(bell, Doorbell { offset: 0x90, value: 0x0001, wide: true }, "VER_{v}");
        assert_eq!(ev, EventRegs { imr: 0x38, isr: 0x3C, wide: true }, "VER_{v}");
    }
}

#[test]
fn masking_is_32_bits_wide_on_the_8125_and_16_on_the_8168() {
    let bar = window();
    bar.present32(0x38, 0xAAAA_AAAA);
    bar.present32(0x40, 0x1234_5678);
    let mut mem = Memory::new();
    let d = mem.driver_with(&bar, RTL8125B);
    mask_and_ack(&d.regs, d.chip.ver);
    assert_eq!((bar.wrote32(0x38), bar.wrote32(0x3C)), (0, 0xFFFF_FFFF));
    let bar = window();
    bar.present32(0x38, 0xAAAA_AAAA);
    bar.present32(0x40, 0x1234_5678);
    let d = mem.driver(&bar);
    mask_and_ack(&d.regs, d.chip.ver);
    assert_eq!((bar.wrote16(0x3C), bar.wrote16(0x3E)), (0, 0xFFFF));
    assert_eq!(bar.wrote32(0x38), 0xAAAA_AAAA, "TxPoll is not touched");
    assert_eq!(bar.wrote32(0x40), 0x1234_5678, "TxConfig is not touched");
}
