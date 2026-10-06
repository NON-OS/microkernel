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

use alloc::vec::Vec;

use super::walked::Walk;
use crate::choose::{choose, Candidate};
use crate::controller::scan_ports;
use crate::engine::park;
use crate::error::{AhciError, AhciResult};
use crate::log::Line;
use crate::setup::Driver;

/// Keep the chosen port and its controller; hand back everything else. Every
/// other port is parked before any controller is released, so no port is
/// still running when its controller's claim goes away.
///
/// With no disk up the attempt fails, so the bring-up schedule runs again: a
/// disk still spinning up, or a link that needed a second COMRESET, gets
/// another chance. The error is the first port that held something and
/// failed, else `NoDisk`; with no controller opened, the first controller's
/// error. Every controller is released by then (dropping `w`).
pub(super) fn serve(mut w: Walk) -> AhciResult<Driver> {
    let cands: Vec<Candidate> = w.probed.iter().map(|p| p.cand).collect();
    let Some(c) = choose(&cands) else {
        if w.opened.is_empty() {
            return Err(w.first_error.unwrap_or(AhciError::DeviceNotFound));
        }
        return Err(w.first_port_error.unwrap_or(AhciError::NoDisk));
    };
    let keep = cands[c.index].controller;
    let block = w.probed.swap_remove(c.index).port;
    for p in w.probed.drain(..) {
        park(p.port, w.opened[p.cand.controller].regs);
    }
    let ctl = w.opened.swap_remove(keep);
    drop(w.opened);
    Line::new()
        .text(b"serving port ")
        .num(u64::from(cands[c.index].port))
        .text(if c.fallback { b" (no NONOS on any disk): " } else { b" (NONOS): " })
        .text(block.names.model())
        .send();
    /*
     * The walk scanned before any port had FIS receive on, when PxSIG still
     * read 0xFFFFFFFF; scan again so port_list reports what the ports are now.
     */
    let ports = scan_ports(ctl.regs, ctl.info.pi, ctl.info.port_count);
    Ok(Driver { handles: ctl.handles, regs: ctl.regs, info: ctl.info, ports, block: Some(block) })
}
