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

/* One process as /proc shows it, under the family's numbers. */

use alloc::vec::Vec;

use crate::linux::file::{self, Proc};
use crate::linux::guest::{Guest, BRK_BASE};

use super::super::pid_ns::PidNs;
use super::facts::{dispositions, image_of, parked};
use super::lend::HOST_NS;

/* One process as /proc shows it, under its numbers in the family's namespace. */
pub(super) fn proc_of(guests: &[Guest], ns: &mut PidNs, j: usize, asking: bool) -> Proc {
    let g = &guests[j];
    let parent = guests.iter().find(|p| p.children.contains(&g.pid)).map(|p| p.pid);
    let exe = image_of(guests, g);
    let (kernel, pgid, sid) = (g.pid, g.pgid, g.sid);
    let sleeping = !asking && parked(g);
    let (cwd, fds, regions) = (g.cwd.clone(), file::open_fds(g), g.regions.clone());
    let (brk, umask) = ((BRK_BASE, g.brk), g.umask);
    let (caught, ignored) = dispositions(g);
    let members: Vec<u32> = [g.pid].iter().chain(g.threads.iter()).copied().collect();
    let tids = members.iter().map(|t| (ns.outward(*t), *t)).collect();
    Proc {
        ns: ns.outward(kernel),
        kernel,
        ppid: parent.map_or(HOST_NS, |p| ns.outward(p)),
        pgid: ns.outward(pgid),
        sid: ns.outward(sid),
        tids,
        sleeping,
        exe,
        cwd,
        fds,
        regions,
        brk,
        umask,
        caught,
        ignored,
    }
}
