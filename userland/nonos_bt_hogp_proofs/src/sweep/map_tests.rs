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
//! The Report Map lied about (HID 1.11, 6.2.2): cut, an octet changed,
//! octets slipped in, or a collection opened again. The parser refuses it
//! or lays it out; a layout it gives decodes any report, of any length,
//! without reading past it, and its LED report is its stated size.

use super::rng::{each, Rng};
use crate::map::maps::{unnumbered, KEYBOARD};
use crate::map::mice::{absolute, mouse, wide};
use nonos_bt_hogp::report::map::{held, leds, parse, pointer, Dir};

fn sound(rng: &mut Rng) -> Vec<u8> {
    match rng.below(5) {
        0 => KEYBOARD.to_vec(),
        1 => unnumbered(),
        2 => mouse(),
        3 => wide(),
        _ => absolute(),
    }
}

#[test]
fn sound_maps_lay_out_and_every_lie_is_refused_or_read_within_bounds() {
    each(|rng| {
        let mut m = sound(rng);
        assert!(parse(&m).is_ok(), "sound");
        let at = rng.below(m.len());
        match rng.below(4) {
            0 => m.truncate(at),
            1 => m[at] ^= 1 + rng.below(255) as u8,
            2 => m.splice(at..at, rng.span(1, 6)).for_each(drop),
            _ => m.splice(at..at, [0xa1, 0x01]).for_each(drop),
        }
        let Ok(layout) = parse(&m) else { return };
        let mut ids: Vec<u8> = layout.ids(Dir::Input).collect();
        ids.extend([0, rng.byte(), rng.byte()]);
        for id in ids {
            let value = rng.span(0, 80);
            let _ = (held(&layout, id, &value), pointer(&layout, id, &value));
            if let Some(out) = leds(&layout, id, rng.byte()) {
                assert_eq!(Some(out.len()), layout.octets(Dir::Output, id), "report {id}");
            }
        }
    });
}
