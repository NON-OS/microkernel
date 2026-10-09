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

/* One process of the family, as /proc shows it. */

use alloc::vec::Vec;

use crate::linux::guest::Region;

use super::super::exe::Exe;
use super::shape::Open;

#[derive(Clone)]
pub struct Proc {
    /* The pid in the family's namespace, and the kernel's behind it. */
    pub ns: u32,
    pub kernel: u32,
    pub ppid: u32,
    pub pgid: u32,
    pub sid: u32,
    /* Every thread's number, the leader first. */
    pub tids: Vec<(u32, u32)>,
    /* Waiting in a call rather than on the CPU. */
    pub sleeping: bool,
    pub exe: Exe,
    pub cwd: Vec<u8>,
    pub fds: Vec<Open>,
    pub regions: Vec<Region>,
    pub brk: (u64, u64),
    pub umask: u16,
    /* Bit n-1 set for each signal n it catches, and for each it ignores. */
    pub caught: u64,
    pub ignored: u64,
    /* What its children used, those it has waited for. */
    pub reaped: super::super::super::cpu::Usage,
}
