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

//! Devices as the ITS knows them: a device id and the interrupt
//! translation table its events are looked up in.

use super::super::msi::MsiError;
use super::encode::cmd::{mapd, unmapd};
use super::memory::{alloc, free};
use super::state::{Device, Its};

impl Its {
    /// The slot holding `id`, mapped first if it has none yet. Functions
    /// behind one bridge can share an id, so a second bind finds the first
    /// one's table and takes other events in it.
    pub(super) fn device(&mut self, id: u32) -> Result<usize, MsiError> {
        if u64::from(id) >= self.device_limit {
            return Err(MsiError::DeviceOutOfRange);
        }
        if let Some(at) = self.find(id) {
            return Ok(at);
        }
        let at = self.devices.iter().position(Option::is_none).ok_or(MsiError::Exhausted)?;
        let bytes = u64::from(self.typer.ite_bytes) << self.event_bits;
        let itt = alloc(bytes, 0x1000).ok_or(MsiError::NoMemory)?;
        // On failure the ITS may hold the table without having said so, so
        // it is kept rather than freed under it.
        self.queue.run(self.regs, &[mapd(id, itt.phys, self.event_bits)])?;
        self.devices[at] = Some(Device { id, itt, events: 0 });
        Ok(at)
    }

    pub(super) fn find(&self, id: u32) -> Option<usize> {
        self.devices.iter().position(|d| d.is_some_and(|d| d.id == id))
    }

    /// Unmap a device whose last event went, then free its table. One the
    /// ITS did not confirm unmapping keeps its record, table and all, for
    /// the next bind to use.
    pub(super) fn retire(&mut self, at: usize) {
        let Some(dev) = self.devices[at] else {
            return;
        };
        if dev.events == 0 && self.queue.run(self.regs, &[unmapd(dev.id)]).is_ok() {
            free(dev.itt);
            self.devices[at] = None;
        }
    }
}
