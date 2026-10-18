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

//! The capsule's register window on an armed host: each access goes to the
//! Sim, which reaches only the DMA grants live at that moment, as the device
//! does through the IOMMU, with the left-over engine shown on top.

use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use super::armed::Armed;
use super::host::Sim;

/// The Software Reset bits that end any DMA: For All, For DAT Line.
const STOPS_DMA: u8 = RESET_ALL | 1 << 2;
const RESET_ALL: u8 = 1 << 0;

thread_local! {
    static PLUGGED: RefCell<Option<Rc<RefCell<Armed>>>> = const { RefCell::new(None) };
}

pub(super) fn plug(armed: Rc<RefCell<Armed>>) {
    PLUGGED.with(|p| *p.borrow_mut() = Some(armed));
}

/// Reach the host plugged in on this thread.
pub fn with<T>(f: impl FnOnce(&mut Armed) -> T) -> T {
    PLUGGED.with(|p| f(&mut p.borrow().as_ref().expect("a model host plugged in").borrow_mut()))
}

impl Armed {
    fn live(&self) -> RefMut<'_, Sim> {
        let mut sim = self.sim.0.borrow_mut();
        sim.mem = nonos_libc::live_dma();
        sim
    }

    pub fn read(&mut self, off: u32, n: usize) -> u32 {
        let v = self.live().read(off, n);
        match off {
            0x24 if self.from.is_some() => v | self.busy,
            0x2f if self.pending != 0 && v as u8 & self.pending == 0 => {
                self.resets += u32::from(self.pending & RESET_ALL != 0);
                self.from = self.from.filter(|_| self.runs_through_reset);
                self.pending = 0;
                v
            }
            _ => v,
        }
    }

    pub fn write(&mut self, off: u32, n: usize, v: u32) {
        if off == 0x2f {
            self.pending |= v as u8 & STOPS_DMA;
        }
        self.live().write(off, n, v);
    }
}
