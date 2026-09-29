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

/* A runtime services table around the firmware's GetVariable, installed in a
 * fresh manager. The table is returned so it outlives the manager's pointer. */

use super::firmware::get_variable;
use super::time::*;
use super::unsupported::*;
use crate::arch::x86_64::uefi::manager::UefiManager;
use crate::arch::x86_64::uefi::tables::RuntimeServices;

pub(super) fn manager_with_firmware() -> (UefiManager, Box<RuntimeServices>) {
    let table = Box::new(RuntimeServices {
        header: unsafe { core::mem::zeroed() },
        get_time,
        set_time,
        get_wakeup_time,
        set_wakeup_time,
        set_virtual_address_map,
        convert_pointer,
        get_variable,
        get_next_variable_name,
        set_variable,
        get_next_high_mono_count,
        reset_system,
        update_capsule,
        query_capsule_capabilities,
        query_variable_info,
    });
    let manager = UefiManager::new();
    *manager.runtime_services.write() = Some(&*table as *const RuntimeServices);
    (manager, table)
}
