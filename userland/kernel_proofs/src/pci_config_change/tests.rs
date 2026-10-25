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

use super::change::{merge, split, MASK_SHIFT};
use super::command_bits::{exact, plain, BUS_MASTER, INTX_DISABLE, MEMORY_SPACE, WRITABLE};

const COMMAND: u32 = 0x04;

/// The register a masked Command write leaves, or `None` where refused.
fn masked(mask: u16, value: u16, current: u16) -> Option<u16> {
    let (register, m) = split(COMMAND | ((mask as u32) << MASK_SHIFT))?;
    assert_eq!((register, m), (COMMAND as u16, mask));
    exact(merge(value, m, current)?, current)
}

#[test]
fn the_plain_form_can_never_clear_a_writable_bit() {
    // The defect: with firmware's bits all writable, no value turns BM off.
    let current = MEMORY_SPACE | BUS_MASTER;
    for new in 0..=WRITABLE {
        if new & !WRITABLE == 0 {
            assert_ne!(plain(new, current).unwrap() & BUS_MASTER, 0, "new {new:#06x}");
        }
    }
}

#[test]
fn the_masked_form_turns_bus_master_off_from_every_register() {
    for current in 0..=u16::MAX {
        let after = masked(BUS_MASTER, 0, current).expect("clearing BM refused");
        assert_eq!(after, current & !BUS_MASTER, "current {current:#06x}");
    }
}

#[test]
fn the_masked_form_sets_and_clears_interrupt_disable() {
    for current in [0, MEMORY_SPACE, INTX_DISABLE | BUS_MASTER, 0x0107 | INTX_DISABLE] {
        assert_eq!(masked(INTX_DISABLE, 0, current), Some(current & !INTX_DISABLE));
        assert_eq!(masked(INTX_DISABLE, INTX_DISABLE, current), Some(current | INTX_DISABLE));
    }
}

#[test]
fn a_masked_write_never_moves_a_bit_outside_the_allowlist() {
    for bit in 0..16 {
        let mask = 1u16 << bit;
        for current in [0u16, 0xFFFF, 0x0107, 0x0400] {
            let changed = masked(mask, 0, current).map(|a| a != current);
            let set = masked(mask, mask, current).map(|a| a != current);
            if mask & WRITABLE == 0 {
                assert!(changed != Some(true) && set != Some(true), "bit {bit}");
            }
        }
    }
}
