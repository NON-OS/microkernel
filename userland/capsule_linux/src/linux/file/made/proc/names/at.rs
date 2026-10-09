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

/* The things a /proc path can name. */

use alloc::vec::Vec;

use super::super::super::view::Proc;

/* Where a /proc path lands. */
pub enum At<'a> {
    Root,
    System(&'a [u8]),
    Sysctl(&'a [&'a [u8]]),
    /* A process's directory, or one of its threads' under task/. */
    PidDir(Option<u32>),
    Task(&'a Proc),
    Pid { proc: &'a Proc, tid: u32, file: &'a [u8] },
    FdDir(&'a Proc),
    Fd { proc: &'a Proc, fd: u32 },
    FdInfoDir(&'a Proc),
    FdInfo { proc: &'a Proc, fd: u32 },
    Link(Vec<u8>),
}
