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

//! A configured device whose logical units are being asked, and where that
//! stands: which unit, which command, what it said, how long the device may
//! still be asked, and when its next command may go (`pace`).

use crate::clock::Deadline;
use crate::disk::Disk;

/// How long a device is asked, all its units together: a USB-SATA bridge or
/// a slow stick can take thirty seconds to come ready, and the kernel's disk
/// selection waits while a device is asked. Each unit had thirty of its own.
const DEVICE_MS: u64 = 30_000;
/// A unit that says it has no medium sooner after configuration is asked
/// again then: a card still starting up says so, and was passed over.
const SETTLE_MS: u64 = 1_000;

#[derive(Clone, Copy)]
pub(super) enum Asked {
    Inquiry,
    Ready { started: bool },
    Capacity,
}

pub struct Asking {
    pub(super) disk: Disk,
    pub(super) last_lun: u8,
    pub(super) asked: Asked,
    /// The digests of the unit's INQUIRY and the device's serial number.
    pub(super) named: Option<u64>,
    pub(super) serial: Option<u64>,
    /// When the device is given up; when no medium is believed; when the
    /// next command may go.
    pub(super) until: Deadline,
    pub(super) settle: Deadline,
    pub(super) due: Deadline,
    /// Commands in a row the transport failed, on any unit.
    pub(super) failed: u8,
}

impl Asking {
    /// The device just configured, from unit `disk.lun`'s INQUIRY.
    pub fn start(disk: Disk, last_lun: u8, serial: Option<u64>) -> Self {
        let (until, settle) = (Deadline::after_ms(DEVICE_MS), Deadline::after_ms(SETTLE_MS));
        let (asked, named, due) = (Asked::Inquiry, None, Deadline::after_ms(0));
        Self { disk, last_lun, asked, named, serial, until, settle, due, failed: 0 }
    }

    pub fn wait_ms(&self) -> u64 {
        self.due.left_ms()
    }
}
