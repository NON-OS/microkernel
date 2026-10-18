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

use nonos_virtio::ModernCaps;

use super::broker::LibcBroker;
use super::choose::modern_caps;
use crate::discover::Found;

/// The capabilities to drive the function with over the modern transport,
/// or `None` for the legacy one (see `choose`).
pub fn probe(dev: &Found, claim_epoch: u64) -> Result<Option<ModernCaps>, &'static str> {
    modern_caps(&dev.bars, &mut LibcBroker::new(dev.device_id, claim_epoch))
}
