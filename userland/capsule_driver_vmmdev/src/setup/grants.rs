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

//! What a bring-up holds of the broker: the claim, the request port, the
//! event page and the request page, given back newest first.

use nonos_libc::{mk_device_release, mk_dma_unmap, mk_mmio_unmap, mk_pio_release};

#[derive(Clone, Copy, Debug, Default)]
pub struct Grants {
    pub device_id: u64,
    pub pio: Option<u64>,
    pub mmio: Option<u64>,
    pub dma: Option<u64>,
}

impl Grants {
    pub fn release(&self) {
        if let Some(g) = self.dma {
            let _ = mk_dma_unmap(g);
        }
        if let Some(g) = self.mmio {
            let _ = mk_mmio_unmap(g);
        }
        if let Some(g) = self.pio {
            let _ = mk_pio_release(g);
        }
        let _ = mk_device_release(self.device_id);
    }
}
