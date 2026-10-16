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
use crate::parts;

pub fn answering(region: &Arc<FakeBar>, answer: Answer) -> Part {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let served = AtomicU16::new(0);
    let (log, link) = (Arc::clone(&seen), parts::attach());
    let live = run(region, move |bar| {
        let _answering = link.step();
        let done = served.load(Ordering::Acquire);
        if serve::step(bar, serve::indices(bar), done, &log, answer, 0) {
            served.store(done.wrapping_add(1), Ordering::Release);
        }
        link.turned();
    });
    Part::new(seen, live)
}

/// A part that also reads the device status in `window`, as a device does:
/// it serves only while DRIVER_OK is set, read after the ring so that the
/// two are of one queue, and both under its step lock (`parts`), so a reset
/// cannot land midway through an answer. It notes the status each command
/// was taken under.
/// What it has served is the used index it wrote, which a restart wipes with
/// the rest of the ring, so it follows the driver through every recovery
/// without having to catch the reset. On each queue it answers the first
/// `limit` commands and leaves the rest unanswered.
pub fn answering_after_reset(
    region: &Arc<FakeBar>,
    window: &Arc<FakeBar>,
    answer: Answer,
    limit: u16,
) -> Part {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let (log, window, link) = (Arc::clone(&seen), Arc::clone(window), parts::attach());
    let live = run(region, move |bar| {
        let _answering = link.step();
        let at = serve::indices(bar);
        let status = window.wrote8(DEVICE_STATUS);
        if status & STATUS_DRIVER_OK != 0 && at.1 < limit {
            serve::step(bar, at, at.1, &log, answer, status);
        }
        link.turned();
    });
    Part::new(seen, live)
}
