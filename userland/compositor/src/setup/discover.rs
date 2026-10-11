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

/// The gfx drivers, best first: a virtio GPU, then the Bochs adapter. Both
/// serve the same NVGP ops, so whichever is up is the display; a machine has
/// one or the other, and a GPU beats a framebuffer copy when it has both.
pub const GFX_SERVICES: [&[u8]; 2] = [b"driver.virtio_gpu0", b"driver.bga0"];

pub struct GfxEndpoint {
    pub port: u32,
}

// Resolves the gfx driver's port through the kernel service registry.
// mk_ipc_call routes by port; the kernel verifies the owning pid is
// alive before delivery, so the compositor does not need to hold it.
pub fn lookup_gfx_endpoint() -> Result<GfxEndpoint, &'static str> {
    GFX_SERVICES.iter().find_map(|name| lookup(name)).ok_or("gfx service not announced")
}

fn lookup(name: &[u8]) -> Option<GfxEndpoint> {
    let mut pid: u32 = 0;
    let mut port: u32 = 0;
    let rc =
        mk_service_lookup(name.as_ptr(), name.len(), &mut port as *mut u32, &mut pid as *mut u32);
    (rc >= 0 && pid != 0 && port != 0).then_some(GfxEndpoint { port })
}
