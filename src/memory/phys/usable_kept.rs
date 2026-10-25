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

//! The boot map's RAM, every region not reserved, unusable, NVS, MMIO or PAL
//! code, the loader's and firmware's own included (L60, L63): an RMRR over
//! the kernel image is as much RAM as one over memory the allocator hands out.

use spin::Mutex;

use super::usable_runs::Runs;

static KEPT: Mutex<Runs> = Mutex::new(Runs::EMPTY);

/// Keep `sorted`, the boot map's RAM regions ordered by start.
pub fn keep(sorted: &[(u64, u64)]) {
    let mut kept = KEPT.lock();
    for &(start, end) in sorted {
        kept.add(start, end);
    }
}

/// Whether `[start, end)` shares a byte with usable RAM, or might: before
/// the map was kept, or past the room kept, the answer is yes.
pub fn touches_usable(start: u64, end: u64) -> bool {
    let kept = KEPT.lock();
    kept.len == 0 || kept.touches(start, end)
}
