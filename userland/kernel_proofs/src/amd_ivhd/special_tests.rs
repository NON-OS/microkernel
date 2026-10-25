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

//! IVHD special devices (L50): the id an I/O APIC or HPET sends under is
//! named once, so AMD-Vi never gives a PCI function with that id a table.

use super::fixture::ivrs;
use super::ivhd_special::specials;

fn read(table: &[u8]) -> (Vec<u16>, bool) {
    let mut out = [0u16; 4];
    let (n, lost) = specials(table, &mut out);
    (out[..n].to_vec(), lost)
}

#[test]
fn an_ioapic_and_an_hpet_are_named_once_and_nothing_else() {
    let entries = [
        0x48, 0x00, 0x00, 0x00, 0x21, 0xA0, 0x00, 0x01, // I/O APIC 33 as 00:14.0
        0x48, 0x00, 0x00, 0x00, 0x00, 0xA2, 0x00, 0x02, // HPET 0 as 00:14.2
        0x42, 0x00, 0x03, 0x00, 0x00, 0x08, 0x02, 0x00, // an alias, not special
        0x02, 0x00, 0x06, 0x00, // a select
    ];
    let mut table = ivrs(0x10, &entries);
    table.extend_from_slice(&ivrs(0x40, &entries)[48..]);
    assert_eq!(read(&table), (vec![0xA0, 0xA2], false));
}

#[test]
fn too_many_say_so_and_a_cut_entry_is_not_read() {
    let many: Vec<u8> = (0..5u8).flat_map(|b| [0x48, 0, 0, 0, b, b, 0x00, 0x01]).collect();
    let (got, lost) = read(&ivrs(0x11, &many));
    assert!(got == [0, 1, 2, 3] && lost);
    let cut = [0x48, 0x00, 0x00, 0x00, 0x21, 0xA0];
    assert_eq!(read(&ivrs(0x10, &cut)), (vec![], false));
}
