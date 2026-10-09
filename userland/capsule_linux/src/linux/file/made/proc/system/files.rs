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

/* /proc's system-wide files, and the family's use behind them. */

use alloc::vec::Vec;

use super::super::super::super::cpu::{self, Usage};
use super::super::super::super::declared::{self as d};
use super::super::super::view::View;
use super::memory::{cpuinfo, meminfo};
use super::stat::stat;
use super::time::{loadavg, uptime};

pub fn content(v: &View, name: &[u8]) -> Option<Vec<u8>> {
    Some(match name {
        b"cpuinfo" => cpuinfo(),
        b"filesystems" => super::super::mounts::filesystems(),
        b"loadavg" => loadavg(v),
        b"meminfo" => meminfo(v),
        b"stat" => stat(v),
        b"uptime" => uptime(v),
        b"version" => {
            [b"Linux version ", d::RELEASE, b" (", d::HOSTNAME, b") ", d::VERSION, b"\n"].concat()
        }
        _ => return None,
    })
}

/* What the family's threads used, and each process's resident memory. */
pub(super) fn family(v: &View) -> Usage {
    let all: Vec<&super::super::super::view::Proc> = v.procs.iter().collect();
    let mut u = cpu::usage(&cpu::threads_of(&all));
    let leaders: Vec<u32> = v.procs.iter().map(|p| p.kernel).collect();
    u.resident_kb = cpu::usage(&leaders).resident_kb;
    u
}
