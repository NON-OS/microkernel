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

//! The register window: the capabilities read from config space, then the
//! common, notify and device regions mapped from them by the shared
//! transport, each checked against what the broker actually mapped. The
//! walk and the mapping used to be the driver's own. virtio-gpu is
//! modern-only (1af4:1050), so a function missing one of the three is
//! refused by the one it lacks.

use nonos_virtio::{map_window, modern_caps, Need, VirtioError, Window};

use super::broker::LibcBroker;
use crate::discover::Found;

/// The scanout and capset counts are read from the device configuration;
/// the driver polls, so ISR is not mapped.
const NEED: Need = Need { isr: false, device: true };

pub fn map(dev: &Found, claim_epoch: u64) -> Result<Window, &'static str> {
    let mut broker = LibcBroker::new(dev.device_id, claim_epoch);
    let caps = modern_caps(&dev.bars, &mut broker, NEED).map_err(VirtioError::message)?;
    let caps = caps.ok_or(VirtioError::NoCommonCfg.message())?;
    map_window(&mut broker, &caps, NEED).map_err(VirtioError::message)
}
