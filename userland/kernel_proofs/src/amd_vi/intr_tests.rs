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

//! K4: AMD interrupt remapping's encodings against Linux amd/iommu.c:
//! set_dte_irq_entry (IV, IntTabLen 9, IntCtl 10b, the table in 51:6),
//! irte_prepare's legacy entry, and build_inv_irt.

use super::command::{command_opcode, invalidate_interrupt_table};
use super::dte::{blocked, keep_interrupts, translated};
use super::intr_rule::{irte, passes_any, remapped, ABORT_ALL, ENTRIES};

#[test]
fn the_interrupt_quadword_matches_linux() {
    let q = remapped(0x1234_5680);
    assert_eq!(q, (2 << 60) | (9 << 1) | 1 | 0x1234_5680);
    assert_eq!(remapped(0x1234_56FF) & 0xFFF, 0x6C0 | (9 << 1) | 1, "the root in bits 51:6");
    assert_eq!(ENTRIES, 1 << 9);
    assert!(!passes_any(q) && !passes_any(ABORT_ALL));
    assert!(passes_any(0), "IV clear passes everything");
    assert!(passes_any(ABORT_ALL | 1 << 58), "NMI pass");
    assert!(passes_any(ABORT_ALL | 0b01 << 60), "IntCtl forward unmapped");
    assert!(!passes_any(blocked()[2]), "a blocked device's interrupts are refused");
}

#[test]
fn a_remapping_entry_matches_linux() {
    // RemapEn bit 0, IntType 0 (fixed), DM 0 (physical), dest 15:8, vector 23:16.
    assert_eq!(irte(0x81, 3), 0x0081_0301);
    for v in 0..=255u8 {
        let e = irte(v, v ^ 0x5A);
        assert!(e & 1 == 1 && (e >> 16) as u8 == v && (e >> 8) as u8 == v ^ 0x5A);
        assert_eq!(e >> 24, 0);
    }
}

#[test]
fn moving_a_device_between_domains_keeps_its_interrupts() {
    let mut dte = translated(0x5000, 4, 7);
    dte[2] = remapped(0x9000);
    assert_eq!(keep_interrupts(blocked(), dte)[2], remapped(0x9000));
    assert_eq!(keep_interrupts(blocked(), dte)[0], blocked()[0]);
    assert_eq!(keep_interrupts(translated(0x6000, 4, 8), blocked())[2], ABORT_ALL);
}

#[test]
fn the_interrupt_table_invalidation_names_the_device() {
    let c = invalidate_interrupt_table(0x00A0);
    assert_eq!(command_opcode(c), 0x5);
    assert_eq!((c[0], c[2], c[3]), (0x00A0, 0, 0));
}
