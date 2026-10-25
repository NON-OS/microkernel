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

//! What one bring-up attempt holds of the broker, and the mode firmware
//! left, given back in that order when the handles drop: the mode registers
//! written back while the ports are still granted, both mappings, the port
//! grant, and the claim last. A capsule killed outright drops nothing; the
//! kernel takes back what it held.

use nonos_libc::{mk_device_release, mk_mmio_unmap, mk_pio_release};

use crate::regs::Regs;
use crate::setup::Saved;

pub struct BrokerHandles {
    device_id: u64,
    pio_grant: u64,
    maps: [u64; 2],
    mapped: usize,
    firmware: Option<(Regs, Saved)>,
}

impl BrokerHandles {
    pub const fn new(device_id: u64, pio_grant: u64) -> Self {
        Self { device_id, pio_grant, maps: [0; 2], mapped: 0, firmware: None }
    }

    /// A mapping made with the claim, given back before the claim is.
    pub fn mapped(&mut self, grant_id: u64) {
        if let Some(slot) = self.maps.get_mut(self.mapped) {
            *slot = grant_id;
            self.mapped += 1;
        }
    }

    /// The mode firmware left in `regs`, read before the driver writes it.
    pub fn restore_at_drop(&mut self, regs: Regs, firmware: Saved) {
        self.firmware = Some((regs, firmware));
    }

    /// The broker's id of the adapter, which a DGON names when it leaves.
    pub fn device_id(&self) -> u64 {
        self.device_id
    }
}

impl Drop for BrokerHandles {
    fn drop(&mut self) {
        if let Some((regs, firmware)) = self.firmware {
            firmware.restore(&regs);
        }
        for &g in self.maps[..self.mapped].iter().rev() {
            let _ = mk_mmio_unmap(g);
        }
        let _ = mk_pio_release(self.pio_grant);
        let _ = mk_device_release(self.device_id);
    }
}
