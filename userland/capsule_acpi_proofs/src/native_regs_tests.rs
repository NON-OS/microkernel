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

//! The power word changes only the power controller and its indicator, for
//! every slot control value; sent masked through MkAcpiSlot, the kernel
//! writes exactly the word the capsule meant; and every status bit it
//! clears is one the kernel lets it clear.

use crate::native::regs::*;
use crate::native::slot_rule::{write_word, CONTROL_BITS, STATUS_EVENTS};
use crate::native::slot_wire::{SLOT_CONTROL, SLOT_STATUS};

#[test]
fn the_power_word_touches_power_and_its_indicator_alone() {
    let fields = POWER_INDICATOR | POWER_OFF;
    for ctrl in 0..=u16::MAX {
        let on = power_word(ctrl, true);
        let off = power_word(ctrl, false);
        assert_eq!(on & !fields, ctrl & !fields, "{ctrl:#x}");
        assert_eq!(off & !fields, ctrl & !fields, "{ctrl:#x}");
        assert_eq!(on & fields, INDICATOR_ON);
        assert_eq!(off & fields, INDICATOR_OFF | POWER_OFF);
    }
}

#[test]
fn the_change_bits_are_the_latched_events_only() {
    assert_eq!(CHANGES & (PRESENT | COMMAND_COMPLETED), 0, "states are never cleared");
    for bit in [BUTTON_PRESSED, POWER_FAULT, MRL_CHANGED, PRESENCE_CHANGED, DLL_CHANGED] {
        assert_ne!(CHANGES & bit, 0);
    }
}

#[test]
fn the_kernel_writes_the_word_the_capsule_meant() {
    for ctrl in 0..=u16::MAX {
        for on in [true, false] {
            let meant = power_word(ctrl, on);
            assert_eq!(write_word(SLOT_CONTROL, u32::from(meant & CONTROL_BITS), ctrl), Ok(meant));
        }
    }
    for clear in [CHANGES, COMMAND_COMPLETED] {
        assert_eq!(clear & !STATUS_EVENTS, 0);
        assert_eq!(write_word(SLOT_STATUS, u32::from(clear), 0xFFFF), Ok(clear));
    }
}
