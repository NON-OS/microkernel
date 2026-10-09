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

use super::fixture::{ivrs, read, span};
use super::ivhd_scope::covers;

/* A client layout: the root complex selected, a range over every bus, and
an IOAPIC special entry (handle 0x21, id 00:14.0, variety 1). */
#[test]
fn select_range_and_special_entries() {
    let entries = [
        0x02, 0x00, 0x00, 0x00, // select 00:00.0
        0x03, 0x08, 0x00, 0x00, // start 00:01.0
        0x04, 0xFF, 0xFF, 0x00, // end ff:1f.7
        0x48, 0x00, 0x00, 0xD7, 0x21, 0xA0, 0x00, 0x01, // IOAPIC at 00:14.0
    ];
    let got = read(&ivrs(0x10, &entries));
    assert_eq!(got, vec![span(0, 0, true), span(8, 0xFFFF, false), span(0xA0, 0xA0, true)]);
    assert!(covers(&got, 0x0300) && covers(&got, 0) && !covers(&got, 0x0005));
}

#[test]
fn alias_entries_name_both_ids_and_hid_entries_their_own() {
    let entries = [
        0x42, 0x00, 0x03, 0x00, 0x00, 0x08, 0x02, 0x00, // 03:00.0 arrives as 02:01.0
        0x43, 0x00, 0x05, 0x00, 0x00, 0x00, 0x04, 0x00, // range from 05:00.0 as 04:00.0
        0x04, 0xFF, 0x05, 0x00, // end 05:1f.7
        0xF0, 0x98, 0x00, 0x00, // ACPI HID device as 00:13.0
        b'A', b'M', b'D', b'I', b'0', b'0', b'4', b'0', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let got = read(&ivrs(0x11, &entries));
    assert_eq!(
        got,
        vec![
            span(0x0300, 0x0300, true),
            span(0x0208, 0x0208, true),
            span(0x0400, 0x0400, true),
            span(0x0500, 0x05FF, false),
            span(0x0098, 0x0098, true),
        ]
    );
}

#[test]
fn all_covers_every_id_and_an_end_without_a_start_adds_nothing() {
    assert_eq!(read(&ivrs(0x40, &[0x01, 0, 0, 0])), vec![span(0, 0xFFFF, false)]);
    assert!(read(&ivrs(0x10, &[0x04, 0xFF, 0xFF, 0])).is_empty());
}
