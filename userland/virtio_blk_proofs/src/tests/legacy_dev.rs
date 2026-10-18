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

//! A legacy virtio-blk behind its port grant, from the specification's
//! legacy register layout: features, the queue's page number, size and
//! selector, the doorbell, the status byte, the ISR, and the configuration
//! (after the two vector registers with MSI-X on). It can start as a dead
//! predecessor left it, DRIVER_OK with its ring live at a page, and take
//! some reads to finish a reset, the old status reading back until then.

use std::cell::RefCell;
use std::rc::Rc;

use crate::constants::STATUS_DRIVER_OK;

#[derive(Default)]
pub struct LegacyBlk {
    pub status: u8,
    pub pfn: u32,
    pub sel: u16,
    pub qnum: u16,
    pub host_features: u32,
    pub guest_features: u32,
    pub capacity: u64,
    /// MSI-X is on: the vector registers sit before the configuration.
    pub msix: bool,
    /// Configuration-change and queue vectors.
    pub vectors: [u16; 2],
    /// Status reads a reset takes to finish; u32::MAX never does.
    pub reset_reads: u32,
    /// Reads left of the reset under way.
    pub resetting: Option<u32>,
    /// A status other than zero was written over a reset still under way.
    pub written_mid_reset: bool,
    /// The driver read the status as zero after it asked for a reset.
    pub seen_stopped: bool,
    pub notified: u32,
    /// Answers each request told with this used length and status.
    pub answers: Option<(u32, u8)>,
}

pub type Dev = Rc<RefCell<LegacyBlk>>;

impl LegacyBlk {
    /// Whether a queue is running: the device is DRIVER_OK with a ring.
    pub fn queue_live(&self) -> bool {
        self.status & STATUS_DRIVER_OK != 0 && self.pfn != 0
    }

    /// Ask for a reset: done at once, or after `reset_reads` status reads.
    pub(super) fn reset_asked(&mut self) {
        self.seen_stopped = false;
        match self.reset_reads {
            0 => self.stopped(),
            n => self.resetting = Some(n),
        }
    }

    pub(super) fn stopped(&mut self) {
        (self.status, self.pfn, self.resetting) = (0, 0, None);
    }
}
