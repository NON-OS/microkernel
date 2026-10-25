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
//! Everything after the register window: the transport, the scanouts, the
//! best-effort probes, and every enabled scanout taken into the
//! wire server.

use nonos_gfx_pool::Pool;
use nonos_gfx_serve::Server;
use nonos_virtio::Window;

use super::{probes, scanouts, transport};
use crate::discover::Found;
use crate::display::take::take;
use crate::driver::Driver;
use crate::state::{FenceCounter, ScanoutTable, Tally};

pub fn run(dev: Found, epoch: u64, window: &Window) -> Result<Driver<Pool>, &'static str> {
    let ready = transport::start(dev.device_id, epoch, window)?;
    let (scanouts, fences) = (ScanoutTable::new(), FenceCounter::new());
    scanouts::seed(&ready.control_queue, &scanouts, &fences)?;
    let (device, notify) = ((dev.device_id, epoch), window.notify_area());
    let probed = probes::run(ready.control_queue, ready.features, notify, &fences, device)?;
    let gpu = take((probed.queue, fences), device, &scanouts, probed.panel)?;
    let server = Server::new(gpu, Pool::default());
    Ok(Driver { device_id: dev.device_id, server, notify, tally: Tally::default() })
}
