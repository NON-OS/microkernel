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

use nonos_libc::{mk_service_lookup, nonos_display_dimensions};

use super::present::GFX_SERVICE;

pub fn primary_dimensions() -> Option<(u32, u32)> {
    let mut width = 0u32;
    let mut height = 0u32;
    let rc = nonos_display_dimensions(0, &mut width, &mut height);
    if rc < 0 || width == 0 || height == 0 {
        return None;
    }
    Some((width, height))
}

/// Whether the virtio-gpu driver has announced itself, asked the way the
/// compositor asks before it presents through it.
pub fn virtio_announced() -> bool {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(
        GFX_SERVICE.as_ptr(),
        GFX_SERVICE.len(),
        &mut port as *mut u32,
        &mut pid as *mut u32,
    );
    rc >= 0 && pid != 0 && port != 0
}
