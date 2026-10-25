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

//! MkAcpiSlot's masks: a value with any bit outside its register's mask is
//! refused whatever the register holds; a value inside it changes only the
//! mask's bits; the read-only registers take no write; and no interrupt
//! enable or interlock bit is ever the capsule's to set.

use super::acpi::slot_rule::{write_word, SlotRefusal, CONTROL_BITS, STATUS_EVENTS};
use super::acpi::slot_wire::{register, SLOT_CONTROL, SLOT_STATUS};

const HELD: [u16; 5] = [0, 0xFFFF, 0x0400, 0x103F, 0xF83F];

#[test]
fn slot_control_takes_only_its_bits_and_keeps_the_rest() {
    for now in HELD {
        for v in 0..=0x1_FFFFu32 {
            let got = write_word(SLOT_CONTROL, v, now);
            if v & !u32::from(CONTROL_BITS) != 0 {
                assert_eq!(got, Err(SlotRefusal::Bits), "{v:#x} refused, not trimmed");
            } else {
                let w = got.unwrap();
                assert_eq!(w & !CONTROL_BITS, now & !CONTROL_BITS, "the rest kept");
                assert_eq!(u32::from(w & CONTROL_BITS), v);
            }
        }
    }
    // Interrupt enables (0-5), interlock (11) and link-state enable (12).
    assert_eq!(CONTROL_BITS & 0x183F, 0);
}

#[test]
fn slot_status_clears_only_events() {
    for now in HELD {
        for v in 0..=0x1_FFFFu32 {
            let got = write_word(SLOT_STATUS, v, now);
            let inside = v & !u32::from(STATUS_EVENTS) == 0;
            assert_eq!(got, if inside { Ok(v as u16) } else { Err(SlotRefusal::Bits) });
        }
    }
}

#[test]
fn nothing_else_is_written() {
    for which in (0..32u64).filter(|w| ![SLOT_CONTROL, SLOT_STATUS].contains(w)) {
        assert_eq!(write_word(which, 0, 0), Err(SlotRefusal::ReadOnly));
    }
    let regs: Vec<_> = (0..32).filter_map(register).collect();
    assert_eq!(regs, [(0x14, 4), (0x12, 2), (0x18, 2), (0x1A, 2)], "four registers, no more");
}
