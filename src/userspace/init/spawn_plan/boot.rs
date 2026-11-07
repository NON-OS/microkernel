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

use super::once::Tried;
use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;
use crate::services::lifecycle::CapsuleState;
use spin::Mutex;

static TRIED: Mutex<Tried> = Mutex::new(Tried::new());

pub(super) fn capsule(
    prefix: &'static str,
    name: &'static str,
    spawn_fn: fn() -> Result<(), SpawnError>,
    state_fn: fn() -> &'static CapsuleState,
) {
    /* Once a boot: the plan's later walks (`late`) spawn only what is new. */
    if !TRIED.lock().first(name) {
        return;
    }
    /* An app the person turned off at setup; nothing else is ever withheld. */
    if super::super::app_choice::capsule_off(name) {
        crate::sys::boot_log::ok(prefix, "turned off at setup, not spawned");
        return;
    }
    super::super::capsule_boot::boot(prefix, name, spawn_fn, state_fn);
}
