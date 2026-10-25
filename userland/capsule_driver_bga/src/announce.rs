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

//! The service, under the name the compositor looks up after virtio-gpu's.

use nonos_libc::mk_service_register;

pub const SERVICE_NAME: &[u8] = b"driver.bga0";
/// Service 4240 and reply 4241, as Capsule.mk declares them. The kernel
/// registers the name on this port when it spawns the capsule
/// (src/hardware/bga_capsule), and accepts the capsule's own registration
/// of it on that port alone.
pub const SERVICE_PORT: u32 = 4240;

/// Whether the kernel took the service. Only the compositor is meant to
/// reach it (C6): the kernel's holder table names who may send to it.
pub fn announce() -> bool {
    mk_service_register(SERVICE_NAME.as_ptr(), SERVICE_NAME.len(), SERVICE_PORT) >= 0
}
