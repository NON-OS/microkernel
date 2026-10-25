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

//! The leave phase of an ordinary power-off or reboot, before the other CPUs
//! stop: every running disk driver made safe within its budget and all of
//! them within `total_ms` (hardware::block_device::leave). Never on the
//! panic path, whose kernel can no longer trust a capsule to answer.

use crate::arch::power::PowerOff;

pub(super) fn run(off: PowerOff, total_ms: u64) {
    let restart = matches!(off, PowerOff::Reboot);
    crate::hardware::block_device::leave::power_off_disks(restart, total_ms);
}
