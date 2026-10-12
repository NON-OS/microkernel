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

//! _WDG blocks read from a buffer the interpreter returns, and GUIDs
//! written the way their publishers write them.

use crate::enc::*;
use crate::run::{eval, raw};
use crate::wmi::{guid_text, parse_wdg, EVENT, METHOD};

fn block(guid: [u8; 16], id: [u8; 2], instances: u8, flags: u8) -> Vec<u8> {
    [&guid[..], &id[..], &[instances, flags]].concat()
}

#[test]
fn blocks_and_their_events() {
    let g1: [u8; 16] = core::array::from_fn(|i| i as u8);
    let g2 = [0xAB; 16];
    let bytes =
        [block(g1, *b"AA", 1, METHOD), block(g2, [0xD0, 0], 1, EVENT), vec![1, 2, 3]].concat();
    let wdg = eval(&ret(&buffer(bytes.len() as u64, &bytes)), vec![]).unwrap();
    let b = parse_wdg(&raw(&wdg));
    assert_eq!(b.len(), 2, "the trailing partial block is not read");
    assert_eq!((b[0].id, b[0].notify_value()), (*b"AA", None));
    assert_eq!((b[1].instances, b[1].notify_value()), (1, Some(0xD0)));
}

#[test]
fn guid_text_matches_the_published_form() {
    let g = [
        0x6A, 0x0F, 0xBC, 0xAB, 0xA1, 0x8E, 0xD1, 0x11, 0x00, 0xA0, 0xC9, 0x06, 0x29, 0x10, 0x00,
        0x00,
    ];
    assert_eq!(guid_text(&g), "ABBC0F6A-8EA1-11D1-00A0-C90629100000");
}
