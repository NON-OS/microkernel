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

//! A store app already running, found by the registry's own record rather
//! than the shell's memory, which a restart or another client may have
//! made stale.

use super::launch_children::new_child;
use super::launch_wake::focus;
use crate::state::launch::Launch;
use crate::state::{Context, Uptime};
use alloc::vec::Vec;
use nonos_libc::mk_service_lookup;

/// A collision names an instance the top-of-launch registry probe missed --
/// another client loaded it in the interim. Re-probe and focus it; if the
/// window is already gone, stay silent rather than cry a false failure.
pub(super) fn focus_running(ctx: &mut Context, name: &[u8]) {
    if let Some(pid) = already_running(name) {
        ctx.store_apps.installed_pids.insert(name.to_vec(), pid);
        if focus(pid) {
            return;
        }
        ctx.store_apps.installed_pids.remove(name);
    }
}

/// Another client -- the terminal's `nox install`, say -- may already have
/// loaded the app, and this shell would know nothing about it. Spawning a
/// second copy then dies in the kernel on the service endpoint the live
/// instance still holds, with no marker and no reply, so ask the registry
/// before loading.
pub(super) fn already_running(name: &[u8]) -> Option<u32> {
    let mut service = Vec::with_capacity(b"app.".len() + name.len());
    service.extend_from_slice(b"app.");
    service.extend_from_slice(name);
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(service.as_ptr(), service.len(), &mut port, &mut pid);
    if rc < 0 || pid == 0 {
        return None;
    }
    Some(pid)
}

/// The app, by the service name packaged apps register, or, once the
/// installer has finished, as the child it made for the shell.
pub(super) fn find(launch: &Launch, now: Uptime) -> Option<u32> {
    if let Some(pid) = already_running(&launch.name) {
        return Some(pid);
    }
    if launch.served() {
        return new_child(&launch.children, launch.elapsed_ms(now));
    }
    None
}
