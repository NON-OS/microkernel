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

//! Boot mouse report edges, by name.

use crate::hid::mouse_report::mouse_event;

#[test]
fn mouse_report_edges() {
    type Got = Option<(i16, i16, i8, u8)>;
    let cases: Vec<(&str, Vec<u8>, Got)> = vec![
        ("empty", vec![], None),
        ("two bytes", vec![1, 1], None),
        ("three quiet bytes", vec![0, 0, 0], None),
        ("most negative", vec![0, 0x80, 0x80], Some((-128, -128, 0, 0))),
        ("most positive", vec![0, 0x7f, 0x7f], Some((127, 127, 0, 0))),
        ("minus one", vec![0, 0xff, 0xff, 0xff], Some((-1, -1, -1, 0))),
        ("wheel most negative", vec![0, 0, 0, 0x80], Some((0, 0, -128, 0))),
        ("every button bit", vec![0xff, 0, 0], Some((0, 0, 0, 0x1f))),
        ("padding bits alone", vec![0xe0, 0, 0], None),
        ("eight bytes", vec![1, 2, 3, 4, 0xff, 0xff, 0xff, 0xff], Some((2, 3, 4, 1))),
    ];
    for (name, raw, want) in cases {
        let got = mouse_event(&raw, 0).map(|e| (e.dx, e.dy, e.dz, e.buttons));
        assert_eq!(got, want, "{name}");
    }
}
