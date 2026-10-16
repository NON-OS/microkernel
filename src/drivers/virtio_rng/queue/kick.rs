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

//! Telling the device a buffer is waiting.

use super::core::RngQueue;
use core::ptr;
use core::sync::atomic::Ordering;

impl RngQueue {
    pub(super) fn kick(&self) {
        core::sync::atomic::fence(Ordering::SeqCst);
        if self.notify_port != 0 {
            // SAFETY: the probe claimed this port for this device.
            unsafe {
                crate::sys::io::outw(self.notify_port, 0);
            }
        } else if self.notify_mmio != 0 {
            // SAFETY: the BAR's notify register, as the kernel reaches it.
            unsafe {
                ptr::write_volatile(
                    crate::arch::paging::kernel_io(self.notify_mmio) as *mut u16,
                    0,
                );
            }
        }
    }
}
