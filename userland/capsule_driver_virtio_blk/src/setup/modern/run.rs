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

//! The modern sequence: the common, notify, ISR and device regions, then
//! the handshake (`live`). A step that fails once the window is mapped
//! resets the device before the caller releases the claim, which takes
//! every grant with it.

use nonos_virtio::common::reset_until;
use nonos_virtio::{map_window, ModernCaps, VirtioError};

use super::super::driver::Driver;
use super::super::step::step;
use super::live::live;
use crate::clock::expired_after;
use crate::constants::RESET_MS;
use crate::discover::Found;
use crate::transport::{LibcBroker, NEED};

pub fn run(dev: Found, claim_epoch: u64, caps: &ModernCaps) -> Result<Driver, &'static str> {
    step("modern-regs");
    let mut broker = LibcBroker::new(dev.device_id, claim_epoch);
    let window = map_window(&mut broker, caps, NEED).map_err(VirtioError::message)?;
    let live = live(dev, claim_epoch, &window);
    if live.is_err() {
        let _ = reset_until(&window.common.region, expired_after(RESET_MS));
    }
    live
}
