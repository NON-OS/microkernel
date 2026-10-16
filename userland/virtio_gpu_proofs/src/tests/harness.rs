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

//! A ring region and a register window, with the control queue over them.

use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_virtio::common::{
    STATUS_ACKNOWLEDGE, STATUS_DRIVER, STATUS_DRIVER_OK, STATUS_FEATURES_OK,
};

use super::model::{modern_window, regs, view, QUEUE_SIZE};
use crate::constants::VQ_REGION_SIZE;
use crate::device::virtqueue::{ControlQueue, QueueLayout};
use crate::regs::Regs;

/// Where in the notify region the control queue's doorbell is.
pub const DOORBELL: usize = 12;

/// The status a device is driven under once the handshake is done.
pub const HANDSHAKE: u8 =
    STATUS_ACKNOWLEDGE | STATUS_DRIVER | STATUS_FEATURES_OK | STATUS_DRIVER_OK;

/// A ring region and the modern window (offering VERSION_1, for a recovery
/// to negotiate). The region's device address is its host address.
pub struct Harness {
    pub region: Arc<FakeBar>,
    pub regs: Arc<FakeBar>,
    pub queue: ControlQueue,
}

pub fn harness() -> Harness {
    let region = Arc::new(FakeBar::new(VQ_REGION_SIZE));
    let window = modern_window(1, QUEUE_SIZE);
    let ring = view(&region, 0, VQ_REGION_SIZE);
    let layout = QueueLayout::new(QUEUE_SIZE, ring, region.base()).expect("a valid layout");
    let r = Regs { doorbell: DOORBELL, ..regs(&window) };
    Harness { region, regs: window, queue: ControlQueue::new(layout, r) }
}
