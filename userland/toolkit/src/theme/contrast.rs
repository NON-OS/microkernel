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

//! Whether two colours are far enough apart to read, in integers.

use super::linear::LINEAR;

/// WCAG 2.1 relative luminance, scaled by 65535 for the table and 10000 for the
/// coefficients, so full white is 655_350_000.
pub fn luminance(colour: u32) -> u32 {
    let ch = |shift: u32| LINEAR[((colour >> shift) & 0xFF) as usize] as u32;
    2126 * ch(16) + 7152 * ch(8) + 722 * ch(0)
}

/*
 * WCAG's ratio is (Lhi + 0.05) / (Llo + 0.05). Comparing it against a floor needs
 * no division: multiply through instead, in 64 bits so the product of a luminance
 * and a floor in tenths cannot wrap.
 *
 * The 0.05 offset is a twentieth of full scale, and full scale here is 655_350_000,
 * so the offset is exactly 32_767_500 with nothing rounded away.
 */
const OFFSET: u64 = 32_767_500;

/// Whether `a` against `b` meets a contrast floor given in tenths, so 45 is the
/// 4.5:1 WCAG 2.1 asks of body text and 30 the 3:1 it asks of large text and of
pub fn at_least(a: u32, b: u32, floor_tenths: u32) -> bool {
    let (la, lb) = (luminance(a) as u64, luminance(b) as u64);
    let (hi, lo) = if la >= lb { (la, lb) } else { (lb, la) };
    (hi + OFFSET) * 10 >= (lo + OFFSET) * floor_tenths as u64
}

/// The body-text floor, WCAG 2.1 success criterion 1.4.3 at level AA.
pub const BODY: u32 = 45;

/// The floor for large text and for anything that is not text: the same criterion,
/// which is where hint text and a hairline are allowed to sit.
pub const LARGE: u32 = 30;
