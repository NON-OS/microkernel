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

//! The disk writer's sink over a device: what turns `nonos_disk`'s sector
//! stream into driver requests. The error carried across is the driver's
//! status, so a receipt can say which sector the device refused and why.

use nonos_disk::{BlockSink, SinkError};

use crate::device::BlockDevice;

pub struct DeviceSink {
    pub device: BlockDevice,
}

impl BlockSink for DeviceSink {
    fn capacity_sectors(&mut self) -> Result<u64, SinkError> {
        Ok(self.device.sectors)
    }

    fn write_at(&mut self, lba: u64, data: &[u8]) -> Result<(), SinkError> {
        self.device.write(lba, data).map_err(|e| SinkError(e.code()))
    }

    fn read_at(&mut self, lba: u64, out: &mut [u8]) -> Result<(), SinkError> {
        self.device.read(lba, out).map_err(|e| SinkError(e.code()))
    }

    fn flush(&mut self) -> Result<(), SinkError> {
        self.device.flush().map_err(|e| SinkError(e.code()))
    }
}
