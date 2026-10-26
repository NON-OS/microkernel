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

use alloc::format;
use core::sync::atomic::AtomicUsize;

use nonos_libc::mk_service_lookup;

use crate::say::say;

// The display drivers, best first, from the kernel's own table: the names
// the compositor tries are the names the kernel holds to it.
#[path = "../../../../src/services/registry/display_hints.rs"]
pub mod display_hints;
#[path = "../../../../src/services/registry/displays.rs"]
pub mod displays;
mod order;

/// The driver the display log last named, none at first.
static SAID: AtomicUsize = AtomicUsize::new(usize::MAX);

#[derive(Clone, Copy)]
pub struct GfxEndpoint {
    pub name: &'static str,
    pub port: u32,
    pub pid: u32,
}

/// The first display driver on the list that is announced now, and on the
/// display log each driver it finds anew: wait_for_setup names the wire
/// path whichever driver serves it, and the GOP framebuffer when it takes
/// that.
pub fn lookup_gfx_endpoint() -> Result<GfxEndpoint, &'static str> {
    let found = (0..displays::DISPLAYS.len()).find_map(|at| Some((at, lookup_at(at)?)));
    if let Some(driver) = order::news(&SAID, found.map(|(at, _)| at)) {
        say(&format!("gfx lookup: {}", driver));
    }
    found.map(|(_, ep)| ep).ok_or("gfx service not announced")
}

/// The driver at `index` in the list, if it is announced now.
pub fn lookup_at(index: usize) -> Option<GfxEndpoint> {
    displays::DISPLAYS.get(index).and_then(|name| lookup(name))
}

// Resolves a driver's port through the kernel service registry. mk_ipc_call
// routes by port; the kernel verifies the owning pid is alive before
// delivery, so the compositor does not need to hold it. The pid tells a
// restarted driver from the one the compositor attached.
pub fn lookup(name: &'static str) -> Option<GfxEndpoint> {
    let mut pid: u32 = 0;
    let mut port: u32 = 0;
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port as *mut u32, &mut pid);
    (rc >= 0 && pid != 0 && port != 0).then_some(GfxEndpoint { name, port, pid })
}
