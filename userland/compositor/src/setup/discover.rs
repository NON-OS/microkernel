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

use nonos_libc::mk_service_lookup;

// The display drivers, best first, from the kernel's own table: the names
// the compositor tries are the names the kernel holds to it.
#[allow(dead_code)] // the kernel's helpers this capsule does not call
#[path = "../../../../src/services/registry/displays.rs"]
mod displays;

pub struct GfxEndpoint {
    pub port: u32,
}

// Resolves the gfx driver's port through the kernel service registry.
// mk_ipc_call routes by port; the kernel verifies the owning pid is
// alive before delivery, so the compositor does not need to hold it.
pub fn lookup_gfx_endpoint() -> Result<GfxEndpoint, &'static str> {
    let found = displays::DISPLAYS.iter().find_map(|name| lookup(name.as_bytes()));
    found.ok_or("gfx service not announced")
}

fn lookup(name: &[u8]) -> Option<GfxEndpoint> {
    let mut pid: u32 = 0;
    let mut port: u32 = 0;
    let rc =
        mk_service_lookup(name.as_ptr(), name.len(), &mut port as *mut u32, &mut pid as *mut u32);
    (rc >= 0 && pid != 0 && port != 0).then_some(GfxEndpoint { port })
}
