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

//! Reset, ACKNOWLEDGE and DRIVER, the features (VERSION_1, and
//! ACCESS_PLATFORM when offered: no device feature is taken), FEATURES_OK,
//! and the event queue. The status queue is left disabled: no LED is lit.

use nonos_virtio::common::QueueSpec;
use nonos_virtio::common::{accept, choose_size, queue_max, setup_queue, start, NotifyArea};
use nonos_virtio::{Mmio, VirtioError};

use crate::constants::queue::{AVAIL_OFF, DESC_OFF, QUEUE_MAX, USED_OFF};

const EVENTQ: u16 = 0;

/// The ring size the device took and the event queue's doorbell offset.
pub fn run(common: &Mmio, notify: NotifyArea, ring: u64) -> Result<(u16, usize), &'static str> {
    start(common).map_err(VirtioError::message)?;
    accept(common, 0).map_err(VirtioError::message)?;
    let size =
        choose_size(queue_max(common, EVENTQ), QUEUE_MAX, 1).map_err(VirtioError::message)?;
    let spec = QueueSpec {
        index: EVENTQ,
        size,
        min_size: 1,
        desc: ring + DESC_OFF as u64,
        driver: ring + AVAIL_OFF as u64,
        device: ring + USED_OFF as u64,
        vector: None,
    };
    let ready = setup_queue(common, &spec, notify).map_err(VirtioError::message)?;
    Ok((ready.size, ready.notify_offset))
}
