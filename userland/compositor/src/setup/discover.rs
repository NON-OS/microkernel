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

mod order;

/// The driver the display log last named, none at first.
static SAID: AtomicUsize = AtomicUsize::new(usize::MAX);

pub struct GfxEndpoint {
    pub port: u32,
}

// Resolves the gfx driver's port through the kernel service registry, in
// `order`'s order, and names on the display log each driver it finds anew:
// wait_for_setup names the virtio path whichever driver serves it, and the
// GOP framebuffer when it takes that. mk_ipc_call routes by port; the
// kernel verifies the owning pid is alive before delivery, so the
// compositor does not need to hold it.
pub fn lookup_gfx_endpoint() -> Result<GfxEndpoint, &'static str> {
    let found = order::first_announced(announced);
    if let Some(driver) = order::news(&SAID, found.map(|(at, _)| at)) {
        say(&format!("gfx lookup: {}", driver));
    }
    let (_, port) = found.ok_or("gfx service not announced")?;
    Ok(GfxEndpoint { port })
}

fn announced(name: &[u8]) -> Option<u32> {
    let mut pid: u32 = 0;
    let mut port: u32 = 0;
    let rc =
        mk_service_lookup(name.as_ptr(), name.len(), &mut port as *mut u32, &mut pid as *mut u32);
    (rc >= 0 && pid != 0 && port != 0).then_some(port)
}
