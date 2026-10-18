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

//! The boot keyboard report (HID 1.11, B.1): modifier bits, a reserved
//! octet, then up to six keys held. Each report says what is held now, so
//! events come from the difference with the last one.

use super::input::Input;
use alloc::vec::Vec;

/// ErrorRollOver, POSTFail and ErrorUndefined (HID Usage Tables 1.4, 10):
/// in any slot they mean the report cannot say which keys are held.
const PHANTOM: core::ops::RangeInclusive<u8> = 0x01..=0x03;

#[derive(Default)]
pub struct Keyboard {
    modifiers: u8,
    keys: [u8; 6],
}

impl Keyboard {
    /// The events in one report, releases before presses. A report of the
    /// wrong length, or a phantom one, changes nothing: it does not say
    /// the keys it leaves out were let go.
    pub fn report(&mut self, raw: &[u8]) -> Vec<Input> {
        let Ok(r) = <&[u8; 8]>::try_from(raw) else { return Vec::new() };
        let keys: [u8; 6] = [r[2], r[3], r[4], r[5], r[6], r[7]];
        if keys.iter().any(|k| PHANTOM.contains(k)) {
            return Vec::new();
        }
        let mut out = Vec::new();
        let (was, now) = (self.modifiers, r[0]);
        for bit in 0..8u8 {
            let (a, b) = (was >> bit & 1, now >> bit & 1);
            if a == 1 && b == 0 {
                out.push(Input::KeyUp(0xe0 + bit));
            }
        }
        let held = |set: &[u8; 6], k: u8| k != 0 && set.contains(&k);
        out.extend(
            self.keys.iter().filter(|&&k| k != 0 && !held(&keys, k)).map(|&k| Input::KeyUp(k)),
        );
        for bit in 0..8u8 {
            if was >> bit & 1 == 0 && now >> bit & 1 == 1 {
                out.push(Input::KeyDown(0xe0 + bit));
            }
        }
        out.extend(
            keys.iter().filter(|&&k| k != 0 && !held(&self.keys, k)).map(|&k| Input::KeyDown(k)),
        );
        (self.modifiers, self.keys) = (now, keys);
        out
    }

    /// Releases for everything held, for when the link goes.
    pub fn release_all(&mut self) -> Vec<Input> {
        self.report(&[0; 8])
    }
}
