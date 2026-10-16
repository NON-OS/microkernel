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
use crate::log::Line;
use crate::regs::Regs;

/// Hand a port back on a controller that keeps serving: stop it, then drop
/// it, which unmaps its DMA regions. Bus Master stays on for the port that
/// is served, so it is the stop that keeps the HBA out of them: with ST, CR,
/// FRE and FR clear a port fetches and posts nothing (AHCI 1.3.1, 10.1.2).
/// Used for a port whose bring-up failed, and for every port brought up but
/// not served on the controller that is kept.
///
/// A port whose engine will not stop may still DMA into those regions, so
/// they are leaked, never handed back to be reused.
pub fn park(port: Port, regs: Regs) {
    if quiet(&port, regs) {
        drop(port);
        return;
    }
    Line::new().text(b"port engine would not stop; its DMA memory is kept").send();
    core::mem::forget(port);
}

/// Hand a port back with its controller, which goes back whole: stop it and
/// leave its regions to the release, which frees them once it reads Bus
/// Master back off. None is unmapped by hand while Bus Master is still on.
pub fn retire(port: Port, regs: Regs) {
    if !quiet(&port, regs) {
        Line::new().text(b"port engine would not stop; the release ends it").send();
    }
    core::mem::forget(port);
}

/// Stop the command engine and FIS receive, then mask and clear the port's
/// interrupts. False when CR or FR did not clear.
fn quiet(port: &Port, regs: Regs) -> bool {
    let stopped = super::stop::stop(regs, port.base);
    // SAFETY: the port's register block lies inside the mapped ABAR.
    unsafe {
        regs.w32(port.base + PORT_IE, 0);
        regs.w32(port.base + PORT_IS, regs.r32(port.base + PORT_IS));
    }
    stopped.is_ok()
}
