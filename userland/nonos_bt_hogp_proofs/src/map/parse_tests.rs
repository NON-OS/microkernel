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

//! A keyboard's Report Map laid out field by field (HID 1.11, 6.2.2):
//! padding moves the fields after it, each report counts its own bits,
//! and a map without IDs puts everything in report 0.

use super::maps::{unnumbered, KEYBOARD};
use nonos_bt_hogp::report::map::{parse, Dir, Span};

#[test]
fn the_keyboard_map_lays_out_two_reports() {
    let l = parse(&KEYBOARD).expect("parses");
    assert_eq!((l.octets(Dir::Input, 1), l.octets(Dir::Output, 1)), (Some(8), Some(1)));
    assert_eq!(l.octets(Dir::Input, 2), Some(2));
    assert_eq!(l.octets(Dir::Input, 0), None);
    let f: Vec<_> = l.fields(Dir::Input, 1).collect();
    assert_eq!(f.len(), 2, "the constant octet is padding, not a field");
    assert_eq!((f[0].bit, f[0].size, f[0].count, f[0].array), (0, 1, 8, false));
    assert_eq!(f[0].spans, [Span { page: 7, min: 0xe0, max: 0xe7 }]);
    assert_eq!((f[1].bit, f[1].size, f[1].count, f[1].array), (16, 8, 6, true));
    assert_eq!((f[1].lmin, f[1].lmax), (0, 0x65));
    let leds: Vec<_> = l.fields(Dir::Output, 1).collect();
    assert_eq!((leds[0].bit, leds[0].count, leds[0].spans[0].page), (0, 5, 8));
    let c: Vec<_> = l.fields(Dir::Input, 2).collect();
    assert_eq!((c[0].bit, c[0].size, c[0].lmax), (0, 16, 0x3ff));
    assert_eq!(c[0].spans, [Span { page: 0x0c, min: 0, max: 0x3ff }]);
}

#[test]
fn a_map_without_ids_is_report_zero() {
    let l = parse(&unnumbered()).expect("parses");
    assert_eq!((l.octets(Dir::Input, 0), l.octets(Dir::Output, 0)), (Some(8), Some(1)));
    assert_eq!(l.ids(Dir::Input).collect::<Vec<_>>(), [0]);
}

#[test]
fn usages_follow_the_list_and_the_last_repeats() {
    let l = parse(&KEYBOARD).unwrap();
    let f = l.fields(Dir::Input, 1).next().unwrap();
    assert_eq!((f.usage_of(0), f.usage_of(7)), (Some((7, 0xe0)), Some((7, 0xe7))));
    assert_eq!(f.usage_of(9), Some((7, 0xe7)), "past the list, the last usage");
    assert_eq!(f.usage(8), None);
}
