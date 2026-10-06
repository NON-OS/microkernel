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

use super::port::Port;
use crate::constants::regs::{PORT_IE, PORT_IS};
use crate::regs::Regs;

/// Hand a port back: stop its command engine and FIS receive so the HBA no
/// longer DMAs into the port's regions, mask and clear its interrupts, then
/// drop the port, which unmaps its DMA regions. Used for every port the
/// driver brought up but does not serve, and for one whose bring-up failed.
///
/// A port whose engine will not stop may still DMA into those regions, so
/// they are leaked, never handed back to be reused.
pub fn park(port: Port, regs: Regs) {
    let stopped = super::stop::stop(regs, port.base);
    unsafe {
        regs.w32(port.base + PORT_IE, 0);
        regs.w32(port.base + PORT_IS, regs.r32(port.base + PORT_IS));
    }
    if stopped.is_err() {
        crate::log::Line::new().text(b"port engine would not stop; its DMA memory is kept").send();
        core::mem::forget(port);
        return;
    }
    drop(port);
}
