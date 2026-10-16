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

//! The driver's side of the rig: the transport over its window, and the
//! queue over its grants, as bring-up builds them for that transport.

use nonos_devmodel::FakeBar;
use nonos_virtio::Mmio;

use super::model::{Rig, DATA_PHYS, HEADER_PHYS, SIZE};
use crate::queue::{Buffer, Queue};
use crate::regs::Regs;
use crate::transport::{Modern, Transport};

/// All of `bar`, as the bounds-checked view the driver is handed.
pub fn view(bar: &FakeBar) -> Mmio {
    part(bar, 0, bar.len())
}

/// `len` bytes of `bar` from `at`.
pub fn part(bar: &FakeBar, at: usize, len: usize) -> Mmio {
    assert!(at + len <= bar.len(), "a view past its window");
    // SAFETY: inside the cells, which live as long as `bar`; every test
    // keeps `bar` until it is done with the view.
    unsafe { Mmio::new((bar.base() as usize + at) as *mut u8, len) }
}

impl Rig {
    pub fn transport(&self) -> Transport {
        if self.legacy {
            return Transport::Legacy(Regs::mmio(view(&self.regs)));
        }
        let common = part(&self.regs, 0, 0x40);
        let (notify, isr) = (part(&self.regs, 0x40, 0x10), part(&self.regs, 0x50, 4));
        Transport::Modern(Modern { common, notify, doorbell: 0, isr })
    }

    pub fn queue(&self) -> Queue {
        let header = Buffer { view: view(&self.header), phys: HEADER_PHYS };
        let data = Buffer { view: view(&self.data), phys: DATA_PHYS };
        let queue = Queue::new(view(&self.ring), SIZE, header, data).expect("grants fit");
        // As `setup::legacy` builds it for the legacy transport.
        if self.legacy {
            queue.on_legacy()
        } else {
            queue
        }
    }
}
