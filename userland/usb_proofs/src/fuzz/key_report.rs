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

//! Keyboard reports a hostile device might send, and the keys the HID
//! keyboard page defines, stated here apart from the driver's own rule.

use crate::rng::Rng;

/// A key as the HID keyboard page defines one, stated here apart from the
/// driver's own rule.
pub fn is_key(usage: u8) -> bool {
    matches!(usage, 0x04..=0xA4 | 0xB0..=0xDD)
}

/// A keyboard report a hostile device might send: mostly whole, slots
/// drawn from a few keys so presses and releases repeat, with error codes,
/// reserved usages, empty slots and any byte mixed in.
pub fn hostile_key_report(rng: &mut Rng, keys: &[u8]) -> Vec<u8> {
    let len = if rng.one_in(10) { rng.below(17) } else { 8 };
    (0..len)
        .map(|slot| {
            if slot < 2 {
                return rng.byte();
            }
            match rng.below(10) {
                0..=3 => rng.pick(keys),
                4 => 0,
                5 if rng.one_in(4) => rng.pick(&[1, 2, 3]),
                5 => 0,
                6 => rng.pick(&[0xA5, 0xAF, 0xDE, 0xE0, 0xE7, 0xE8, 0xFF]),
                _ => rng.byte(),
            }
        })
        .collect()
}
