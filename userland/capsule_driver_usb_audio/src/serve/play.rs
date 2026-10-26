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

//! What the driver plays and through what.

use super::queue::PcmQueue;
use crate::uac::Outputs;

/// The device bound, as OUTPUT_STATUS reports it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Device {
    pub ids: (u16, u16),
    pub outputs: Outputs,
}

pub struct Play {
    pub queue: PcmQueue,
    pub running: bool,
    pub device: Option<Device>,
}

impl Play {
    /// Made once, by the run loop; `Default` (fresh.rs) is the same.
    pub fn new() -> Self {
        Self { queue: PcmQueue::new(), running: false, device: None }
    }

    /// The device went: the stream stops, and what it had not played is
    /// dropped rather than played into the next device found.
    pub fn lost(&mut self) {
        (self.device, self.running) = (None, false);
        self.queue.clear();
    }
}
