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

//! IVHD aliases (L47): a device whose interrupts reach the unit under
//! another id is named, and so is that id (L49), so AMD-Vi never gives
//! either a remapping table that would serve the other's vectors.

use super::fixture::ivrs;
use super::ivhd_alias::{aliased, aliases, Alias, MAX_ALIASES};

fn read(table: &[u8]) -> (Vec<Alias>, bool) {
    let mut out = [Alias { first: 0, last: 0, alias: 0 }; MAX_ALIASES];
    let (n, lost) = aliases(table, &mut out);
    (out[..n].to_vec(), lost)
}

#[test]
fn a_select_and_a_range_are_named_and_nothing_else() {
    let entries = [
        0x42, 0x00, 0x03, 0x00, 0x00, 0x08, 0x02, 0x00, // 03:00.0 arrives as 02:01.0
        0x43, 0x00, 0x05, 0x00, 0x00, 0x00, 0x04, 0x00, // 05:00.0 on, as 04:00.0
        0x04, 0xFF, 0x05, 0x00, // end 05:1f.7
        0x02, 0x00, 0x06, 0x00, // select 06:00.0, no alias
    ];
    let (got, lost) = read(&ivrs(0x10, &entries));
    let want = [Alias { first: 0x300, last: 0x300, alias: 0x208 }];
    assert_eq!(&got[..1], &want);
    assert_eq!(got[1], Alias { first: 0x500, last: 0x5FF, alias: 0x400 });
    assert!(!lost && got.len() == 2);
    assert!(aliased(&got, 0x300) && aliased(&got, 0x5A3) && !aliased(&got, 0x600));
    // The alias id's table would take the aliased device's vectors too.
    assert!(aliased(&got, 0x208) && aliased(&got, 0x400));
}

#[test]
fn a_repeated_alias_counts_once_and_a_self_alias_is_none() {
    let entries = [0x42, 0x00, 0x03, 0x00, 0x00, 0x08, 0x02, 0x00]; // 03:00.0 as 02:01.0
    let mut table = ivrs(0x10, &entries);
    table.extend_from_slice(&ivrs(0x11, &entries)[48..]);
    let (got, lost) = read(&table);
    assert!(got == [Alias { first: 0x300, last: 0x300, alias: 0x208 }] && !lost);
    assert!(!aliased(&[Alias { first: 0x700, last: 0x700, alias: 0x700 }], 0x700));
    let range = [Alias { first: 0x700, last: 0x7FF, alias: 0x700 }];
    assert!(aliased(&range, 0x700) && aliased(&range, 0x7A1) && !aliased(&range, 0x800));
}

#[test]
fn a_plain_start_closes_an_open_alias_and_too_many_say_so() {
    let entries = [
        0x43, 0x00, 0x05, 0x00, 0x00, 0x00, 0x04, 0x00, // aliased start, never ended
        0x03, 0x00, 0x07, 0x00, // plain start
        0x04, 0xFF, 0x07, 0x00, // its end
    ];
    let (got, lost) = read(&ivrs(0x11, &entries));
    assert!(got.is_empty() && !lost, "the plain range is not aliased");
    let many: Vec<u8> = (0..=MAX_ALIASES as u8)
        .flat_map(|b| [0x42, 0x00, b, 0x00, 0x00, 0x00, 0x02, 0x00])
        .collect();
    let (got, lost) = read(&ivrs(0x10, &many));
    assert_eq!(got.len(), MAX_ALIASES);
    assert!(lost);
}
