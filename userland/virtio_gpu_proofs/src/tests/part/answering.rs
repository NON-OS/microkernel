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

//! Starting the part on its own thread against a ring region.

use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::{Arc, Mutex};

use nonos_devmodel::{run, FakeBar};
use nonos_virtio::common::{DEVICE_STATUS, STATUS_DRIVER_OK};

use super::{serve, Answer, Part};

pub fn answering(region: &Arc<FakeBar>, answer: Answer) -> Part {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let served = Arc::new(AtomicU16::new(0));
    let log = Arc::clone(&seen);
    let live = run(region, move |bar| serve::step(bar, &served, &log, answer));
    Part::new(seen, live)
}

/// A part that also reads the device status in `window`, as a device does:
/// it serves only once DRIVER_OK is set, and a reset (DRIVER_OK clear)
/// starts its count over, so it follows the driver through a recovery.
pub fn answering_after_reset(region: &Arc<FakeBar>, window: &Arc<FakeBar>, answer: Answer) -> Part {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let served = Arc::new(AtomicU16::new(0));
    let (log, window) = (Arc::clone(&seen), Arc::clone(window));
    let live = run(region, move |bar| {
        if window.wrote8(DEVICE_STATUS) & STATUS_DRIVER_OK == 0 {
            served.store(0, Ordering::Release);
        } else {
            serve::step(bar, &served, &log, answer);
        }
    });
    Part::new(seen, live)
}
