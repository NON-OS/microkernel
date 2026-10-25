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

//! The lock LEDs in report mode: the output report whose fields carry LED
//! page usages (HUT 1.4, 11: 1 Num Lock, 2 Caps Lock, 3 Scroll Lock,
//! 4 Compose, 5 Kana), one bit each. The lock bits are those of the boot
//! output report, so each usage takes bit `usage - 1` of them.

use super::bits;
use super::keys::LEDS;
use super::layout::Layout;
use super::span::Dir;
use alloc::vec;
use alloc::vec::Vec;

/// Output report `id` with the LEDs of `locks` lit, if it has any LED.
pub fn leds(layout: &Layout, id: u8, locks: u8) -> Option<Vec<u8>> {
    let mut out = vec![0; layout.octets(Dir::Output, id)?];
    let mut any = false;
    for f in layout.fields(Dir::Output, id).filter(|f| !f.array && f.size == 1) {
        for i in 0..f.count {
            let Some((LEDS, u @ 1..=5)) = f.usage_of(i) else { continue };
            any = true;
            if locks >> (u - 1) & 1 == 1 {
                bits::set(&mut out, f.bit + i);
            }
        }
    }
    any.then_some(out)
}
