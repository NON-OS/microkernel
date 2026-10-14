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
//! W2c: a link pins its network to WPA3 when the join ran SAE (AKM 8, or 24
//! with an extended key) or message 3 set Transition Disable bit 0, read from
//! the reply's last byte; a reply without that byte reads as no bitmap.

use crate::wifi::join_wire::parse_link;

fn reply(akm: u8, td: Option<u8>) -> Vec<u8> {
    let mut b = vec![1, 2, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 4];
    b.extend_from_slice(b"Home");
    b.extend_from_slice(&[akm, 0, 1, 0, 0, 0, 0]);
    b.extend(td);
    b
}

#[test]
fn the_pin_is_set_each_way_and_not_otherwise() {
    for (akm, td, pins) in [(8, 0, true), (24, 0, true), (2, 1, true), (6, 3, true)]
        .into_iter()
        .chain([(2, 0, false), (6, 0, false), (2, 0x0E, false), (0, 0, false)])
    {
        let l = parse_link(&reply(akm, Some(td))).unwrap();
        assert_eq!((l.transition_disable, l.pins_wpa3()), (td, pins), "akm {akm} td {td}");
        assert_eq!(l.pins_wpa3_on(b"Home"), pins);
        assert!(!l.pins_wpa3_on(b"Other"), "another network's name");
    }
}

#[test]
fn a_link_down_or_an_old_reply_pins_only_by_its_akm() {
    let mut down = reply(8, Some(1));
    down[0] = 0;
    assert!(!parse_link(&down).unwrap().pins_wpa3(), "not associated");
    for (akm, pins) in [(2, false), (8, true)] {
        let old = parse_link(&reply(akm, None)).unwrap();
        assert_eq!((old.transition_disable, old.pins_wpa3()), (0, pins), "akm {akm}");
    }
    let full = reply(2, Some(1));
    for n in 0..full.len() {
        if let Some(l) = parse_link(&full[..n]) {
            assert!(!l.pins_wpa3(), "cut at {n}: a WPA2 link pins only with the whole bitmap");
        }
    }
}
