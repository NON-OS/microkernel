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

//! The bodies the shell passes to the compositor unchanged: ACCESSIBILITY
//! takes the display and four u32 values, ACCESSIBILITY_GET answers the four.

use super::knobs::{Knobs, FILTERS, MAG_STEPS, OUTPUTS};

pub fn body(output: u32, k: &Knobs) -> [u8; 20] {
    let mut b = [0u8; 20];
    for (i, v) in [output, k.filter, k.contrast, k.magnify, k.follow].iter().enumerate() {
        b[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
    }
    b
}

/// The four values as answered, or None for any out of its range.
pub fn knobs(b: &[u8]) -> Option<Knobs> {
    let u = |i: usize| Some(u32::from_le_bytes(b.get(i * 4..i * 4 + 4)?.try_into().ok()?));
    let k = Knobs { filter: u(0)?, contrast: u(1)?, magnify: u(2)?, follow: u(3)? };
    let ok = k.filter < FILTERS && k.contrast < 2 && MAG_STEPS.contains(&k.magnify) && k.follow < 2;
    (ok && b.len() == 16).then_some(k)
}

/// The body ACCESSIBILITY_GET takes: the display.
pub fn ask(output: u32) -> Option<[u8; 4]> {
    (output < OUTPUTS).then(|| output.to_le_bytes())
}
