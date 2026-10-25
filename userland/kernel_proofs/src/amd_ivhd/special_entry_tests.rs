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

//! K4: IVHD special entries (48h): each I/O APIC and HPET by handle and the
//! requester id its interrupts arrive under, in table order, from well formed
//! and hostile tables.

use super::fixture::ivrs;
use super::ivhd_special::{special_entries, Special, HPET, IOAPIC, MAX_SPECIALS};

fn special(handle: u8, id: u16, variety: u8) -> [u8; 8] {
    let [lo, hi] = id.to_le_bytes();
    [0x48, 0xD7, 0, 0, handle, lo, hi, variety]
}

fn read(table: &[u8]) -> Vec<Special> {
    let mut out = [Special { variety: 0, handle: 0, id: 0 }; MAX_SPECIALS];
    let n = special_entries(table, &mut out);
    out[..n].to_vec()
}

#[test]
fn ioapic_and_hpet_are_kept_by_handle() {
    let mut entries = special(0x21, 0x00A0, IOAPIC).to_vec();
    entries.extend_from_slice(&[0x02, 0, 0x08, 0]); // a select entry between
    entries.extend_from_slice(&special(0x00, 0x00A2, HPET));
    let got = read(&ivrs(0x40, &entries));
    assert_eq!(got[0], Special { variety: IOAPIC, handle: 0x21, id: 0x00A0 });
    assert_eq!(got[1], Special { variety: HPET, handle: 0, id: 0x00A2 });
    assert_eq!(got.len(), 2);
}

#[test]
fn one_named_by_two_blocks_counts_once() {
    let mut table = ivrs(0x10, &special(0x21, 0x00A0, IOAPIC));
    let again = ivrs(0x11, &special(0x21, 0x00A0, IOAPIC));
    table.extend_from_slice(&again[48..]);
    assert_eq!(read(&table).len(), 1);
}

#[test]
fn hostile_tables_end_the_walk() {
    let mut cut = ivrs(0x40, &special(0x21, 0x00A0, IOAPIC));
    cut.truncate(cut.len() - 3);
    assert!(read(&cut).is_empty(), "an entry past its block");
    let mut long = ivrs(0x40, &special(0x21, 0x00A0, IOAPIC));
    long[50] = 0xFF;
    assert!(read(&long).is_empty(), "a block length past the table");
    let many: Vec<u8> = (0..40).flat_map(|i| special(i, i as u16, IOAPIC)).collect();
    assert_eq!(read(&ivrs(0x40, &many)).len(), MAX_SPECIALS);
}
