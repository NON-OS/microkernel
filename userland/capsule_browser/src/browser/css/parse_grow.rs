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

/* Factors past this all mean "take nearly everything" (flex-grow: 9999
 * is the idiom); clamping keeps the weighted sums in range. */
const MAX_FACTOR: f32 = 10_000.0;

/* A flex-grow or flex-shrink factor in hundredths, so 0.5 and 1.5 keep
 * their weight against 1. A negative or non-numeric factor is rejected. */
pub(super) fn parse_grow(value: &str) -> Option<u32> {
    let f = value.trim().parse::<f32>().ok()?;
    (f.is_finite() && f >= 0.0).then(|| (f.min(MAX_FACTOR) * 100.0 + 0.5) as u32)
}
