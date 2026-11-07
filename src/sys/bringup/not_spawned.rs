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

//! A driver init does not start, said once: the boot log's
//! "<why>, not spawned" line, and the same as a bring-up event. Init walks
//! its plan again when the ACPI capsule publishes late, so a driver still
//! without its device is said once a boot, not at every walk.

use super::Ev;
use alloc::vec::Vec;
use spin::Mutex;

static SAID: Mutex<Vec<&'static str>> = Mutex::new(Vec::new());

pub fn not_spawned(prefix: &'static str, why: &'static str) {
    {
        let mut said = SAID.lock();
        if said.contains(&prefix) {
            return;
        }
        said.push(prefix);
    }
    crate::sys::boot_log::ok(prefix, &alloc::format!("{why}, not spawned"));
    super::event(prefix, Ev::Absent(why));
}
