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

//! One function brought up: claim, bus mastering, the capability walk,
//! the common, notify and device regions, the event queue's grant, then
//! the handshake and the axes; `ready` hands the ring over. Nothing is
//! bound: the driver polls, and the available ring asks for no interrupt.

use nonos_libc::mk_device_release;
use nonos_virtio::{map_window, parse, ConfigSpace, Need, VirtioError};

use super::input::{abandon, Input};
use super::{claim, dma, pci, ready, LibcBroker};
use crate::discover::Found;
use crate::init::{axes, handshake};

pub fn run(dev: Found) -> Result<Input, &'static str> {
    let epoch = claim::claim(dev.device_id)?;
    let release = |e: &'static str| {
        let _ = mk_device_release(dev.device_id);
        e
    };
    pci::enable(dev.device_id, epoch).map_err(release)?;
    let mut broker = LibcBroker::new(dev.device_id, epoch);
    let cfg = ConfigSpace::read(&mut broker).ok_or(VirtioError::ConfigRead.message());
    let caps = parse(&cfg.map_err(release)?, &dev.bars);
    let need = Need { isr: false, device: true };
    let window =
        map_window(&mut broker, &caps, need).map_err(VirtioError::message).map_err(release)?;
    let dma = dma::map(dev.device_id, epoch).inspect_err(|_| {
        abandon(dev.device_id, epoch, &window, None);
    })?;
    let fail = |e: &'static str| {
        abandon(dev.device_id, epoch, &window, Some(dma.grant_id));
        e
    };
    let common = window.common.region;
    let device = window.device.ok_or(VirtioError::NoDeviceCfg.message()).map_err(fail)?.region;
    let (size, doorbell) =
        handshake::run(&common, window.notify_area(), dma.device_addr).map_err(fail)?;
    let axes = axes::read(&common, device).map_err(fail)?;
    Ok(ready::finish(window, &dma, size, doorbell, axes))
}
