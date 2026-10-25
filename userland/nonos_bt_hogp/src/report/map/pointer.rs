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

//! A pointer's input report (HID 1.11, 6.2.2.5): buttons 1 to 3 on the
//! Button page, one bit each, and relative X, Y and Wheel on the Generic
//! Desktop page (HUT 1.4, 4 and 12). Each value is clamped to what a boot
//! mouse report could carry, one signed octet, so a report-mode pointer
//! can never move the session further in one report than a boot one.

use super::bits;
use super::field::Field;
use super::layout::Layout;
use super::span::Dir;

pub const DESKTOP: u16 = 0x01;
pub const BUTTONS: u16 = 0x09;
const X: u16 = 0x30;
const Y: u16 = 0x31;
const WHEEL: u16 = 0x38;
/// The boot mouse report's range for each axis (HID 1.11, B.2).
const RANGE: (i64, i64) = (-128, 127);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pointer {
    pub buttons: u8,
    pub dx: i16,
    pub dy: i16,
    pub wheel: i16,
}

fn axis(f: &Field, usage: u16) -> bool {
    !f.array && f.relative && f.spans.iter().any(|s| s.hits(DESKTOP, usage, usage))
}

/// Whether input report `id` lays out a relative pointer: X and Y.
pub fn points(layout: &Layout, id: u8) -> bool {
    let has = |u| layout.fields(Dir::Input, id).any(|f| axis(f, u));
    has(X) && has(Y)
}

/// The pointer state in a report, or `None` if it is shorter than its
/// layout. A value outside its field's logical extent counts as none.
pub fn pointer(layout: &Layout, id: u8, value: &[u8]) -> Option<Pointer> {
    let value = value.get(..layout.octets(Dir::Input, id)?)?;
    let (mut p, mut sums) = (Pointer::default(), [0i64; 3]);
    for f in layout.fields(Dir::Input, id).filter(|f| !f.array) {
        for i in 0..f.count {
            let Some(v) = bits::element(f, i, value)? else { continue };
            match f.usage_of(i) {
                Some((BUTTONS, n @ 1..=3)) if f.size == 1 && v != 0 => p.buttons |= 1 << (n - 1),
                Some((DESKTOP, u @ (X | Y | WHEEL))) if f.relative => {
                    sums[usize::from(u - X).min(2)] += v;
                }
                _ => {}
            }
        }
    }
    let clamp = |v: i64| v.clamp(RANGE.0, RANGE.1) as i16;
    (p.dx, p.dy, p.wheel) = (clamp(sums[0]), clamp(sums[1]), clamp(sums[2]));
    Some(p)
}
