// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

// The .S sources, in one block so the macros come before their use:
// the system registers, the set/way walk over them, then the entry.

core::arch::global_asm!(
    include_str!("sysregs.S"),
    include_str!("walk.S"),
    include_str!("enter_kernel.S"),
);

extern "C" {
    /// Turns the caches and the MMU off and branches to `entry` with
    /// x0 = `dtb` and x1 = `handoff`.
    ///
    /// # Safety
    /// Boot services have exited; `entry` is the kernel's physical entry,
    /// and the image, the device tree and the handoff are cleaned to the
    /// point of coherency. Nothing comes back.
    pub fn nonos_arm_enter_kernel(entry: u64, dtb: u64, handoff: u64) -> !;
}
