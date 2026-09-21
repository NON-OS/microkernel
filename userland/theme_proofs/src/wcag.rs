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

//! The contrast formula as WCAG 2.1 writes it, in floating point.

/*
 * The capsule computes this in integers off a table, because it has no floating
 * point and cannot afford a pow in a paint. This is the specification it is
 * measured against: written out here from the standard, independently of the
 * implementation, so agreement between the two means something.
 *
 * If both were the same arithmetic there would be nothing to prove.
 */
fn linear(channel: u8) -> f64 {
    let c = channel as f64 / 255.0;
    if c <= 0.03928 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// WCAG 2.1 relative luminance.
pub fn luminance(colour: u32) -> f64 {
    let ch = |shift: u32| linear(((colour >> shift) & 0xFF) as u8);
    0.2126 * ch(16) + 0.7152 * ch(8) + 0.0722 * ch(0)
}

/// The contrast ratio between two colours, always at least 1.0.
pub fn ratio(a: u32, b: u32) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    let (hi, lo) = if la >= lb { (la, lb) } else { (lb, la) };
    (hi + 0.05) / (lo + 0.05)
}

/// Success criterion 1.4.3 at AA, for body text.
pub const BODY: f64 = 4.5;

/// The same criterion for large text and for anything that is not text.
pub const LARGE: f64 = 3.0;
