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

//! Legacy or modern, decided once the device is claimed (config space is
//! only readable with the claim's epoch).

use nonos_virtio::{choose, parse, wants_probe, ConfigSpace, Kind, ModernCaps, VirtioError};

use super::broker::LibcBroker;
use crate::discover::Found;

/// The capabilities to drive the function with when it is to be driven
/// modern; `None` keeps the legacy path. A transitional function with its
/// legacy I/O BAR is not read at all, so a boot without an IOMMU takes the
/// legacy path exactly as before.
pub fn probe(dev: &Found, claim_epoch: u64) -> Result<Option<ModernCaps>, &'static str> {
    if !wants_probe(dev.pci_device, &dev.bars) {
        return Ok(None);
    }
    let mut broker = LibcBroker::new(dev.device_id, claim_epoch);
    let cfg = ConfigSpace::read(&mut broker).ok_or(VirtioError::ConfigRead.message())?;
    let caps = parse(&cfg, &dev.bars);
    Ok(match choose(dev.pci_device, &dev.bars, &caps) {
        Kind::Modern => Some(caps),
        Kind::Legacy => None,
    })
}
