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

use nonos_virtio::{modern_caps, ModernCaps, Need, VirtioError};

use super::broker::LibcBroker;
use crate::discover::Found;

/// The structures the modern bring-up maps besides common and notify: the
/// device configuration, for the capacity, and the ISR byte, which lowers
/// the INTx line when MSI-X cannot be had. The specification has every
/// virtio-pci function present all four.
pub const NEED: Need = Need { isr: true, device: true };

/// The capabilities to drive the function with over the modern transport,
/// or `None` for the legacy one. Modern whenever the function presents
/// it, a transitional 1af4:1001 too: only the modern transport can take
/// ACCESS_PLATFORM (plan/50 C3), which `common::accept` takes whenever it
/// is offered and never assumes. A transitional function with its I/O BAR
/// used to be driven legacy without its capabilities being read; a
/// modern-only 1af4:1042 missing a structure is refused by name.
pub fn probe(dev: &Found, claim_epoch: u64) -> Result<Option<ModernCaps>, &'static str> {
    let mut broker = LibcBroker::new(dev.device_id, claim_epoch);
    modern_caps(&dev.bars, &mut broker, NEED).map_err(VirtioError::message)
}
