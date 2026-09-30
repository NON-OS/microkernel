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

use super::walk::Walk;
use crate::choose::{choose, Candidate};
use crate::controller::scan_ports;
use crate::engine::park;
use crate::setup::Driver;

/// Keep the chosen port and its controller; hand back everything else. Every
/// other port is parked before any controller is released, so no port is
/// still running when its controller's claim goes away. `None` when the walk
/// opened no controller.
pub(super) fn serve(mut w: Walk) -> Option<Driver> {
    let cands: Vec<Candidate> = w.probed.iter().map(|p| p.cand).collect();
    let choice = choose(&cands);
    let (keep, block) = match choice {
        Some(c) => (cands[c.index].controller, Some(w.probed.swap_remove(c.index).port)),
        None if w.opened.is_empty() => return None,
        None => (0, None),
    };
    for p in w.probed.drain(..) {
        park(p.port, w.opened[p.cand.controller].regs);
    }
    let ctl = w.opened.swap_remove(keep);
    drop(w.opened);
    /*
     * The walk scanned before any port had FIS receive on, when PxSIG still
     * read 0xFFFFFFFF; scan again so port_list reports what the ports are now.
     */
    let ports = scan_ports(ctl.regs, ctl.info.pi, ctl.info.port_count);
    Some(Driver { handles: ctl.handles, regs: ctl.regs, info: ctl.info, ports, block })
}
