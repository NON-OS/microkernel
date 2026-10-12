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

//! GTDT's timers, its GT blocks with their frames, and its watchdogs.

use super::build::*;
use super::gtdt_build::*;
use crate::arm_acpi::gtdt::gtdt;
use crate::arm_acpi::sdt::TableError;

#[test]
fn timers_blocks_and_watchdogs() {
    let g = gtdt(&table(3, &[gt_block(2), watchdog()])).unwrap();
    assert_eq!((g.cnt_control_base, g.cnt_read_base), (0x2A43_0000, 0x2A80_0000));
    assert_eq!((g.nonsecure_el1.gsiv, g.virtual_el1.gsiv, g.el2.gsiv), (30, 27, 26));
    assert!(g.nonsecure_el1.active_low() && !g.nonsecure_el1.edge());
    assert_eq!(g.virtual_el2.map(|t| t.gsiv), Some(28));
    assert_eq!(g.blocks[0].frames.len(), 2);
    assert_eq!((g.blocks[0].frames[1].number, g.blocks[0].frames[1].physical.gsiv), (1, 58));
    assert_eq!((g.watchdogs[0].refresh_frame, g.watchdogs[0].timer.gsiv), (0x2A44_0000, 93));
    assert_eq!(
        gtdt(&table(2, &[])).unwrap().virtual_el2,
        None,
        "revision 2 has no EL2 virtual timer"
    );
}

#[test]
fn platform_timers_that_do_not_fit() {
    let mut nine = gt_block(1);
    nine[12..16].copy_from_slice(&u32le(9));
    assert_eq!(gtdt(&table(2, &[nine])), Err(TableError::Structure(96)), "more than eight frames");
    let mut frames_out = gt_block(1);
    frames_out[16..20].copy_from_slice(&u32le(40));
    assert_eq!(gtdt(&table(2, &[frames_out])), Err(TableError::Structure(96)));
    let mut short_dog = watchdog();
    short_dog[1..3].copy_from_slice(&u16le(20));
    assert_eq!(gtdt(&table(2, &[short_dog])), Err(TableError::Structure(96)));
    let mut inside_fixed = table(2, &[watchdog()]);
    inside_fixed[92..96].copy_from_slice(&u32le(40));
    seal(&mut inside_fixed);
    assert_eq!(gtdt(&inside_fixed), Err(TableError::Structure(40)), "offset inside the fixed part");
}
