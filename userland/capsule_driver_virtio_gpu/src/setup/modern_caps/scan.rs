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

//! Where the modern structures live, from the vendor capabilities.
//!
//! The walk used to read every field of every vendor capability through a
//! config read each, let the last capability of a type win, read the notify
//! multiplier whatever the capability's length, and take a region in any
//! BAR the capability named. It now reads config space once and hands it to
//! the parse the other virtio drivers use (`nonos_virtio::caps`), proved in
//! virtio_transport_proofs: the first usable capability of each type wins,
//! cap_len is checked before a field past it is read, a capability naming
//! an absent, I/O or out-of-range BAR, a range past its BAR, or the pages
//! of the MSI-X table is skipped, pointers below 0x40 end the walk, and the
//! walk stops after 48 capabilities or on a loop.

use nonos_virtio::{parse, ConfigSpace};

use super::broker::LibcBroker;
use super::types::{ModernCaps, Region};
use crate::discover::Found;

pub fn read(dev: &Found, epoch: u64) -> Result<ModernCaps, &'static str> {
    let mut broker = LibcBroker::new(dev.device_id, epoch);
    let cfg = ConfigSpace::read(&mut broker).ok_or("virtio-gpu: pci config read failed")?;
    let caps = parse(&cfg, &dev.bars);
    Ok(ModernCaps {
        common: caps.common.map(region).ok_or("virtio-gpu: common cfg missing")?,
        notify: caps.notify.map(region).ok_or("virtio-gpu: notify cfg missing")?,
        device: caps.device.map(region).ok_or("virtio-gpu: device cfg missing")?,
        notify_multiplier: caps.notify_multiplier,
    })
}

fn region(r: nonos_virtio::Region) -> Region {
    Region { bar: r.bar, offset: r.offset, length: r.length }
}
