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

//! The live windows of an app, as the service registry knows them. The
//! registry answers one exact name at a time, so each of an app's instance
//! names (state::instance) is asked in turn; looking up the base name alone
//! missed every window the dock opened after the first.

use super::launcher_request::lookup_pid;
use crate::state::instance::{app_index_of, instance_name, INSTANCE_NAME_MAX, INSTANCE_SLOTS};
use crate::state::LAUNCHER_APPS;

/// Call `found` with the pid of each live instance of `base`, base name first,
/// until it returns true. Returns whether it did.
pub fn each_live_pid(base: &[u8], mut found: impl FnMut(u32) -> bool) -> bool {
    let mut buf = [0u8; INSTANCE_NAME_MAX];
    for slot in 0..=INSTANCE_SLOTS {
        let Some(name) = instance_name(base, slot, &mut buf) else { break };
        if let Some(pid) = lookup_pid(name) {
            if found(pid) {
                return true;
            }
        }
    }
    false
}

/// The launcher app whose window process `pid` is, whichever instance it is.
pub fn app_of_pid(pid: u32) -> Option<usize> {
    let mut buf = [0u8; INSTANCE_NAME_MAX];
    for app in LAUNCHER_APPS.iter() {
        for slot in 0..=INSTANCE_SLOTS {
            let Some(name) = instance_name(app.service, slot, &mut buf) else { break };
            if lookup_pid(name) == Some(pid) {
                return app_index_of(LAUNCHER_APPS.iter().map(|a| a.service), name);
            }
        }
    }
    None
}
