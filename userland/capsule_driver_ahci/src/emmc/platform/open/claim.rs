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

//! The device claim and register window grant, given back on drop.

use nonos_libc::{mk_device_release, mk_mmio_unmap};

use super::super::env::SerialLog;
use super::command::command;

/// The claim and the register window. Dropped once the host is stopped, it
/// turns bus mastering off, unmaps the window and releases the claim; the
/// release frees the DMA regions after the broker reads Bus Master back off.
pub(super) struct Claim {
    pub(super) device_id: u64,
    pub(super) epoch: u64,
    pub(super) mmio_grant: Option<u64>,
}

impl Drop for Claim {
    fn drop(&mut self) {
        // The broker ORs Command bits until K6, so this "off" write changes
        // nothing yet and the release's own stop is what holds; it is written
        // so the order holds on its own once the broker stores the word.
        let _ = command(&SerialLog, self.device_id, self.epoch, false);
        if let Some(g) = self.mmio_grant {
            let _ = mk_mmio_unmap(g);
        }
        let _ = mk_device_release(self.device_id);
    }
}
