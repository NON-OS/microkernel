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

//! Which ports take hot plug, and the power word: it changes only the
//! power controller and its indicator, for every slot control value.

use crate::native::regs::*;

#[test]
fn only_root_and_downstream_ports_with_a_capable_slot() {
    let flags = |kind: u16, slot: bool| kind << 4 | if slot { 1 << 8 } else { 0 };
    for kind in 0..16u16 {
        let port = kind == 4 || kind == 6;
        assert_eq!(hot_plug_port(flags(kind, true), HOT_PLUG_CAPABLE), port, "kind {kind}");
        assert!(!hot_plug_port(flags(kind, false), HOT_PLUG_CAPABLE), "no slot, kind {kind}");
        assert!(!hot_plug_port(flags(kind, true), !HOT_PLUG_CAPABLE), "not capable, {kind}");
    }
}

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
