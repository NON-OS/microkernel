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

//! An adapter that may show a mode other than the one written.

use std::cell::Cell;
use std::rc::Rc;

use nonos_devmodel::FakeBar;
use nonos_libc::on_w16;

use crate::constants::DISPI_INDEX_YRES;
use crate::dispi::dispi_off;

/// Count every register write; when `lie`, an adapter that shows the mode
/// written one line shorter, as one that clamps it would.
pub fn adapter(bar: &FakeBar, lie: bool, writes: Rc<Cell<u32>>) {
    let (base, yres) = (bar.base(), dispi_off(DISPI_INDEX_YRES) as u64);
    on_w16(move |addr, value| {
        writes.set(writes.get() + 1);
        if lie && addr == base + yres {
            // SAFETY: the window outlives the case; this is the device's side.
            unsafe { core::ptr::write_volatile(addr as *mut u16, value.wrapping_sub(1)) };
        }
    });
}
