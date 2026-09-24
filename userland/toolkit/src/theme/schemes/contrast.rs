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

//! What the desktop looks like when contrast matters more than taste.

use super::Scheme;

/*
 * High contrast replaces the scheme rather than adjusting it.
 *
 * Lightening a palette until it passes a ratio is the tempting version and the
 * wrong one: the scheme's own relationships decide which pairs move, so one theme
 * ends up at 7:1 and another at 4.6:1, and the setting means something different
 * depending on what it was switched on over. A reader who turns this on cannot see
 * the difference well enough to know which they got.
 *
 * So there is one high-contrast scheme and it is the same from every starting
 * point: black ground, white text, white hairlines that are actually visible, and
 * yellow for the accent because it stays distinguishable from white under the
 * common forms of colour blindness where a blue or green accent does not.
 *
 * Every pair here is 21:1 or the accent's 19.6:1 against black, well past the 7:1
 * that WCAG 2.1 asks of AAA body text.
 */
pub const HIGH_CONTRAST: Scheme = Scheme {
    bg: 0xFF00_0000,
    surface: 0xFF00_0000,
    accent: 0xFFFF_FF00,
    text: 0xFFFF_FFFF,
    border: 0xFFFF_FFFF,
};
