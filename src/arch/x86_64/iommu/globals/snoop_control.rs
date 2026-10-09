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

use core::sync::atomic::Ordering;

use super::state::SNOOP_CONTROL;

/*
 * ECAP.SC. Bit 11 of a second-level leaf asks the unit to snoop CPU caches,
 * and is a reserved bit on a unit that does not report snoop control: every
 * access through such an entry faults (reason 0xC) instead of reaching memory.
 */
pub fn snoop_control() -> bool {
    SNOOP_CONTROL.load(Ordering::Acquire)
}

// Record what the probed unit reported. Set once, before any table is built.
pub fn set_snoop_control(ecap: u64) {
    SNOOP_CONTROL.store(ecap & (1 << 7) != 0, Ordering::Release);
}
