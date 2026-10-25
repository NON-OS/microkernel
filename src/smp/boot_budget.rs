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
//! The boot CPU's deadlines in counter ticks. Host-checked by path from
//! `kernel_proofs`.

/// Counter ticks for `ms` milliseconds at `hz`, or `fallback` when the rate is
/// not known (zero) or too low to express one millisecond.
pub const fn budget_ticks(ms: u64, hz: u64, fallback: u64) -> u64 {
    let ticks = (hz / 1000).saturating_mul(ms);
    if ticks == 0 {
        fallback
    } else {
        ticks
    }
}
