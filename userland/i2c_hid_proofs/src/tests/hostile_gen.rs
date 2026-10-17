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

//! Report descriptors a hostile pad might send: a real one mutated, a run
//! of plausible items with any data, four-byte items above all, or any
//! bytes at all.

use nonos_i2cmodel::touchpad::report_descriptor;

use super::descs::{pad_with_mouse, KEYBOARD, MOUSE};
use super::rng::Rng;

/// Prefixes real descriptors use, the four-byte globals, the stack's and
/// the long item's.
const PREFIXES: [u8; 24] = [
    0x05, 0x06, 0x09, 0x0B, 0x19, 0x29, 0x15, 0x16, 0x25, 0x26, 0x75, 0x77, 0x95, 0x97, 0x81, 0x91,
    0xB1, 0xA1, 0xC0, 0x85, 0x87, 0xA4, 0xB4, 0xFE,
];

pub fn hostile_descriptor(rng: &mut Rng) -> Vec<u8> {
    match rng.below(4) {
        0 | 1 => mutated(rng),
        2 => items(rng),
        _ => {
            let len = rng.below(600);
            rng.bytes(len)
        }
    }
}

fn mutated(rng: &mut Rng) -> Vec<u8> {
    let real = [report_descriptor(), pad_with_mouse(), KEYBOARD.to_vec(), MOUSE.to_vec()];
    let mut d = real[rng.below(real.len())].clone();
    for _ in 0..1 + rng.below(4) {
        let at = rng.below(d.len() + 1);
        match rng.below(4) {
            0 if at < d.len() => d[at] ^= 1 << rng.below(8),
            1 if at < d.len() => d[at] = rng.byte(),
            2 => d.insert(at, rng.byte()),
            _ => d.truncate(at),
        }
    }
    d
}

fn items(rng: &mut Rng) -> Vec<u8> {
    let mut d = Vec::new();
    for _ in 0..rng.below(120) {
        let prefix = if rng.one_in(8) { rng.byte() } else { rng.pick(&PREFIXES) };
        d.push(prefix);
        let size =
            if prefix == 0xFE { 2 + rng.below(4) } else { [0, 1, 2, 4][usize::from(prefix & 3)] };
        let data = rng.bytes(size);
        d.extend(data.iter().map(|&b| if rng.one_in(3) { b & 0x0F } else { b }));
    }
    d
}
