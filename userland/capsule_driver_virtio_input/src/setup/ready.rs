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

//! The last step: every buffer handed to the device, DRIVER_OK, and a kick
//! so the device knows the queue is no longer empty.

use nonos_libc::DmaMapOut;
use nonos_virtio::common::driver_ok;
use nonos_virtio::map::Window;

use super::input::Input;
use crate::events::Report;
use crate::init::Axes;
use crate::queue::EventRing;

pub fn finish(window: Window, dma: &DmaMapOut, size: u16, doorbell: usize, axes: Axes) -> Input {
    let mut ring = EventRing::new(dma.user_va, dma.device_addr, size);
    ring.prime();
    let common = window.common.region;
    driver_ok(&common);
    let input =
        Input { window, notify: window.notify.region, doorbell, ring, axes, report: Report::new() };
    input.kick();
    input
}
