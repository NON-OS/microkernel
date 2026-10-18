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

//! The rig: a register window (the legacy registers, or the modern common,
//! notify and ISR structures), a ring, a request header and a data buffer
//! in host memory, on the transport a test names (`rig.rs`, `device.rs`).

use std::rc::Rc;

use nonos_devmodel::FakeBar;

use crate::constants::{DATA_BUF_LEN, HEADER_BUF_LEN};
use crate::queue::ring_bytes;

pub use super::rig::view;

/// Any power of two serves the tests.
pub const SIZE: u16 = 8;
pub const IRQ: u64 = 3;
pub const HEADER_PHYS: u64 = 0x10_0000;
pub const DATA_PHYS: u64 = 0x20_0000;

#[derive(Clone)]
pub struct Rig {
    pub regs: Rc<FakeBar>,
    pub ring: Rc<FakeBar>,
    pub header: Rc<FakeBar>,
    pub data: Rc<FakeBar>,
    /// Legacy: used lengths are the device's word only as far as the status.
    pub legacy: bool,
}

impl Rig {
    pub fn legacy() -> Self {
        Self::on(true)
    }

    pub fn modern() -> Self {
        Self::on(false)
    }

    fn on(legacy: bool) -> Self {
        let ring = ring_bytes(SIZE).expect("a power of two");
        Rig {
            regs: Rc::new(FakeBar::new(0x60)),
            ring: Rc::new(FakeBar::new(ring)),
            header: Rc::new(FakeBar::new(HEADER_BUF_LEN as usize)),
            data: Rc::new(FakeBar::new(DATA_BUF_LEN as usize)),
            legacy,
        }
    }
}
