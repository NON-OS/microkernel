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

/* /proc/<pid>/status. */

use alloc::string::String;
use alloc::vec::Vec;

use super::super::super::view::Proc;
use super::super::pid_files::{usage, vsize};

pub fn status(p: &Proc, tid: u32) -> Vec<u8> {
    let u = usage(p, tid);
    let comm = core::str::from_utf8(&p.exe.comm).unwrap_or("");
    let state = if p.sleeping { "S (sleeping)" } else { "R (running)" };
    let kb = |n: u64| alloc::format!("{:8} kB", n / 1024);
    let stack: u64 = p
        .regions
        .iter()
        .filter(|r| r.at + r.len == crate::linux::guest::STACK_TOP)
        .map(|r| r.len)
        .sum();
    let data: u64 =
        p.regions.iter().filter(|r| r.write).map(|r| r.len).sum::<u64>().saturating_sub(stack);
    let fdsize = p.fds.iter().map(|f| f.fd + 1).max().unwrap_or(0).div_ceil(64).max(1) * 64;
    let mut s = String::new();
    s += &alloc::format!("Name:\t{comm}\nUmask:\t{:04o}\nState:\t{state}\n", p.umask);
    s += &alloc::format!(
        "Tgid:\t{}\nNgid:\t0\nPid:\t{tid}\nPPid:\t{}\nTracerPid:\t0\n",
        p.ns,
        p.ppid
    );
    s += "Uid:\t0\t0\t0\t0\nGid:\t0\t0\t0\t0\n";
    s += &alloc::format!("FDSize:\t{fdsize}\nGroups:\t\n");
    s += &alloc::format!(
        "NStgid:\t{}\nNSpid:\t{tid}\nNSpgid:\t{}\nNSsid:\t{}\n",
        p.ns,
        p.pgid,
        p.sid
    );
    s += &alloc::format!("VmSize:\t{}\nVmRSS:\t{:8} kB\n", kb(vsize(p)), u.resident_kb);
    s += &alloc::format!("VmData:\t{}\nVmStk:\t{}\n", kb(data), kb(stack));
    s += &alloc::format!("Threads:\t{}\n", p.tids.len());
    s += &alloc::format!(
        "SigBlk:\t{:016x}\nSigIgn:\t{:016x}\nSigCgt:\t{:016x}\n",
        0,
        p.ignored,
        p.caught
    );
    s += "CapInh:\t0000000000000000\nCapPrm:\t0000000000000000\nCapEff:\t0000000000000000\n";
    s += "CapBnd:\t0000000000000000\nCapAmb:\t0000000000000000\nNoNewPrivs:\t1\nSeccomp:\t0\n";
    s += "Cpus_allowed:\t1\nCpus_allowed_list:\t0\n";
    s.into_bytes()
}
