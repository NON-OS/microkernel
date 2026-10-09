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

/*
 * The wall and monotonic clocks MkTimeMillis and MkTimeMonotonic read must
 * keep counting past the point where the counter times a thousand outgrows
 * 64 bits.
 *
 * The kernel's tick to millisecond scale is included by path. It multiplied
 * in 64 bits, so after about 71 days at 3 GHz the product overflowed and a
 * kernel with overflow checks panicked on the next clock read, while one
 * without them wrapped the clock back towards zero. The checks below fail
 * against that code.
 */

#[path = "../../../../src/sys/clock/core/scale.rs"]
pub mod scale;
mod tests;
