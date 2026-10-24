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

use nonos_gfx_pool::Pool;
use nonos_libc::{mk_exit, mk_service_register};

use crate::setup::Driver;

/// The service was refused: the kernel would not register the name.
const EXIT_UNANNOUNCED: i32 = 3;

pub const SERVICE_NAME: &[u8] = b"driver.vmware_svga0";
/// Service 4280 and reply 4281, as Capsule.mk declares them.
pub const SERVICE_PORT: u32 = 4280;

/// Whether the kernel took the service. Only the compositor is meant to
/// reach it: the kernel's holder table names who may send to it.
pub fn announce() -> bool {
    mk_service_register(SERVICE_NAME.as_ptr(), SERVICE_NAME.len(), SERVICE_PORT) >= 0
}

/// `driver`, once its service is taken. The compositor finds the display
/// only through the service, so on a refusal the adapter is given back in
/// the firmware's mode (the handles' drop) before the capsule leaves.
pub fn or_leave(driver: Driver<Pool>) -> Driver<Pool> {
    if !announce() {
        drop(driver);
        mk_exit(EXIT_UNANNOUNCED);
    }
    driver
}
