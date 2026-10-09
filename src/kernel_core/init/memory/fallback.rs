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

use crate::memory::addr::PhysAddr;

/* Fixed spans tried in turn when the firmware's map gave nothing usable. */
pub(super) fn init_fallback() {
    let regions = [
        (0x100000u64, 0x8000_0000u64),
        (0x100000u64, 0x4000_0000u64),
        (0x200000u64, 0x1000_0000u64),
    ];
    for (start, end) in regions {
        if crate::memory::phys::init(PhysAddr::new(start), PhysAddr::new(end)).is_ok() {
            crate::sys::serial::println(b"[MEM] fallback OK");
            return;
        }
    }
}
