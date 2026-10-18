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

//! How long this CPU may stay idle before its own timer wants it.

use crate::arch::aarch64::timer::generic::{current_count, ticks_to_microseconds};
use crate::arch::aarch64::timer::PhysicalTimer;

/// Microseconds until this CPU's timer fires; unbounded when it is not set.
pub(super) fn budget_us() -> u64 {
    if !PhysicalTimer::is_armed() {
        return u64::MAX;
    }
    ticks_to_microseconds(PhysicalTimer::get_compare_value().saturating_sub(current_count()))
}
