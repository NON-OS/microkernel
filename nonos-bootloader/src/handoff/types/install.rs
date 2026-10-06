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

use super::constants::flags;
use super::system::Module;

/*
 * What the loader leaves for an install: the loader image and the kernel
 * image file it verified (the two regions an installer writes to a disk),
 * the partition the loader was read from (so the installer can leave that
 * disk out), and whether the person chose "Install NONOS" in the boot menu. The
 * regions reach the kernel as the handoff's module table, the request as
 * `flags::INSTALL_REQUESTED` in the handoff's flags. The boot profile the
 * menu resolved to rides beside it as its `flags::PROFILE_*` bit. The boot
 * evidence the kernel checks this loader with rides in the same module table:
 * the TCG log, the loader's trailer and the boot-root record. So does the
 * package store, read whole through the firmware's disk driver, and the
 * live plan's model files when the machine has the memory for them.
 */
#[derive(Copy, Clone, Default)]
pub struct InstallHandoff {
    pub source: [Module; 3],
    pub evidence: [Module; 3],
    pub store: Module,
    pub mirror: Module,
    pub requested: bool,
    pub profile: u64,
}

impl InstallHandoff {
    /* The handoff flag bits the menu's choice sets, or none. */
    pub const fn handoff_flag(&self) -> u64 {
        let install = if self.requested { flags::INSTALL_REQUESTED } else { 0 };
        install | self.profile
    }
}
