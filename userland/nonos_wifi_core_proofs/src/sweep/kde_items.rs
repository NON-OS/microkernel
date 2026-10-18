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
//! One Key Data item of each kind, sound.

use super::kde::Kind;
use super::rng::Rng;

/// One item of a random kind, sound.
pub fn item(rng: &mut Rng) -> (Kind, Vec<u8>) {
    let ieee =
        |t: u8, data: Vec<u8>| [&[0xdd, 4 + data.len() as u8, 0, 15, 172, t][..], &data].concat();
    match rng.below(8) {
        0 => (Kind::Rsne, [&[48, 20][..], &rng.bytes(20)].concat()),
        1 => (Kind::Rsnxe, [&[244, 1][..], &[rng.byte()]].concat()),
        2 => (Kind::Gtk, ieee(1, rng.span(3, 30))),
        3 => (Kind::Igtk, ieee(9, rng.span(9, 30))),
        4 => (Kind::Bigtk, ieee(14, rng.span(9, 30))),
        5 => {
            let data = rng.span(0, 3);
            (
                Kind::Disable,
                [&[0xdd, 4 + data.len() as u8, 0x50, 0x6f, 0x9a, 0x20][..], &data].concat(),
            )
        }
        6 => {
            let content = [&[1, 2, 3][..], &rng.span(0, 20)].concat();
            (Kind::Other, [&[0xdd, content.len() as u8][..], &content].concat())
        }
        _ => {
            let id = rng.pick(&[1u8, 3, 50, 127, 200]);
            let body = rng.span(0, 20);
            (Kind::Other, [&[id, body.len() as u8][..], &body].concat())
        }
    }
}
