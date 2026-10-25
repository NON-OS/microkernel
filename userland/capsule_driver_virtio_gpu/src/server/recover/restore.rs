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
//! One recovery try (`tries`): the device reset and its queue restarted on
//! the same grant with the features settled again, then every output shown
//! again on its own surface (`display::reshow`).

use nonos_gfx_serve::Platform;

use crate::driver::Driver;
use crate::init::restart;

pub fn attempt<P: Platform>(driver: &mut Driver<P>) -> Result<(), &'static str> {
    let gpu = &mut driver.server.display;
    let (queue, _) = restart(&gpu.q, driver.notify)?;
    gpu.q = queue;
    gpu.reshow().map_err(|_| "virtio-gpu: outputs not shown again")
}
