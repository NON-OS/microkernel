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

use super::command::{command_opcode, invalidate_interrupt_table};
use super::dte::{blocked, passthrough, translated};
use super::identity_change::is_passthrough;
use super::intr_format::{
    irte, keep_interrupts, msi_data, remapped_quadword, remaps, same_dma, ENTRIES, LEN_LOG2,
};

const TABLE: u64 = 0x0000_0001_2345_6000;
/// InitPass 184, EIntPass 185, NMIPass 186, Lint0Pass 190, Lint1Pass 191.
const PASS_BITS: u64 = (0b111 << 56) | (0b11 << 62);

#[test]
fn a_remapped_device_passes_no_init_nmi_extint_or_lint() {
    let q = remapped_quadword(TABLE);
    assert_eq!(q & 1, 1, "IV");
    assert_eq!((q >> 1) & 0xF, LEN_LOG2 as u64, "IntTabLen");
    assert_eq!(q & 0x000F_FFFF_FFFF_FFC0, TABLE, "root");
    assert_eq!((q >> 60) & 0b11, 0b10, "IntCtl remapped");
    assert_eq!(q & PASS_BITS, 0);
    assert_eq!(1usize << LEN_LOG2, ENTRIES);
}

#[test]
fn no_domain_move_turns_remapping_off() {
    let remapped = [0, 0, remapped_quadword(TABLE), 0];
    for dma in [blocked(), passthrough(), translated(0x7000, 4, 9)] {
        let after = keep_interrupts(dma, remapped);
        assert!(remaps(after));
        assert!(same_dma(after, dma));
        assert_eq!(is_passthrough(after), is_passthrough(dma));
    }
}

#[test]
fn an_entry_is_fixed_physical_and_names_only_its_vector_and_cpu() {
    for vector in 0..=255u8 {
        for dest in [0u8, 1, 0x7F, 0xFF] {
            let e = irte(vector, dest);
            assert_eq!(e & 1, 1, "RemapEn");
            assert_eq!(e & 0b1111_1110, 0, "SupIOPF, IntType, RqEoi, DM, GuestMode");
            assert_eq!((e >> 8) as u8, dest);
            assert_eq!((e >> 16) as u8, vector);
            assert_eq!(e >> 24, 0);
        }
    }
}

#[test]
fn the_message_carries_only_the_index_and_the_flush_names_the_device() {
    for index in 0..ENTRIES as u16 {
        assert_eq!(msi_data(index), index as u32);
    }
    assert_eq!(msi_data(0xFFFF), 0x7FF);
    let c = invalidate_interrupt_table(0x00A8);
    assert_eq!((c[0], command_opcode(c)), (0x00A8, 5));
}
