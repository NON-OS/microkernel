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

//! A common configuration's feature pages and status byte, enough for the
//! shared transport's feature negotiation.

use std::cell::Cell;

use nonos_virtio::common::{CommonCfg, DEVICE_FEATURE, DEVICE_FEATURE_SELECT};
use nonos_virtio::common::{DEVICE_STATUS, DRIVER_FEATURE, DRIVER_FEATURE_SELECT};

#[derive(Default)]
pub struct Common {
    offered: u64,
    page: Cell<u32>,
    driver_page: Cell<u32>,
    pub taken: Cell<u64>,
    status: Cell<u8>,
}

impl Common {
    pub fn offering(offered: u64) -> Self {
        Self { offered, ..Self::default() }
    }
}

impl CommonCfg for Common {
    fn r8(&self, off: usize) -> u8 {
        self.status.get() * u8::from(off == DEVICE_STATUS)
    }
    fn r16(&self, _: usize) -> u16 {
        0
    }
    fn r32(&self, off: usize) -> u32 {
        let page = (self.offered >> (32 * self.page.get().min(1))) as u32;
        page * u32::from(off == DEVICE_FEATURE)
    }
    fn w8(&self, off: usize, v: u8) {
        if off == DEVICE_STATUS {
            self.status.set(v);
        }
    }
    fn w16(&self, _: usize, _: u16) {}
    fn w32(&self, off: usize, v: u32) {
        match off {
            DEVICE_FEATURE_SELECT => self.page.set(v),
            DRIVER_FEATURE_SELECT => self.driver_page.set(v),
            DRIVER_FEATURE => {
                let shift = 32 * self.driver_page.get().min(1);
                let kept = self.taken.get() & !(0xFFFF_FFFFu64 << shift);
                self.taken.set(kept | u64::from(v) << shift);
            }
            _ => {}
        }
    }
}
