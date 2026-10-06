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

/* How much of an off app's mark still shows: enough to read, plainly not live. */
const OFF_ALPHA_PCT: u32 = 35;

/* `argb` with its alpha cut to OFF_ALPHA_PCT percent, the colour kept. */
pub const fn dim(argb: u32) -> u32 {
    let alpha = (argb >> 24) * OFF_ALPHA_PCT / 100;
    (alpha << 24) | (argb & 0x00FF_FFFF)
}
