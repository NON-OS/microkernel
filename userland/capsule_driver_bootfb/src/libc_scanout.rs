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

//! The scanout's two questions over the broker (G-80, L79): whether the
//! boot framebuffer is still this driver's, and letting the claim go once
//! it is not (`loss`).

use nonos_libc::{mk_boot_framebuffer, mk_device_release, BootFramebufferInfo};

use crate::geometry::INFO_LEN;

/// Whether the boot framebuffer is still this driver's: its 24 bytes.
pub fn holds(device_id: u64) -> bool {
    let mut info = BootFramebufferInfo::default();
    mk_boot_framebuffer(device_id, &mut info) == INFO_LEN as i64
}

/// The lost claim let go of; -ENODEV (already gone) changes nothing.
pub fn release(device_id: u64) {
    let _ = mk_device_release(device_id);
}
