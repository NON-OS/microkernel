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

/*
 * What the system is and what a process has used, and who it runs as;
 * then the file calls in table_data.
 */

use crate::linux::abi::nr_path as np;
use crate::linux::call;
use crate::linux::guest::Guest;

pub fn sys_ops(guest: &mut Guest, tid: u32, nr: u64, a: [u64; 6]) -> Option<u64> {
    if let Some(v) = super::table_data::data_ops(guest, nr, a) {
        return Some(v);
    }
    Some(match nr {
        np::SYSINFO => call::sysinfo(guest, a[0]),
        np::GETRUSAGE => call::getrusage(guest, tid, a[0], a[1]),
        np::TIMES => call::times(guest, a[0]),
        np::GETGROUPS => call::getgroups(a[0]),
        np::SETGROUPS => call::setgroups(),
        np::GETRESUID | np::GETRESGID => call::getres(guest, [a[0], a[1], a[2]]),
        np::SETRESUID | np::SETRESGID => call::setres([a[0], a[1], a[2]]),
        np::GETPRIORITY => call::getpriority(guest, a[0], a[1]),
        np::SETPRIORITY => call::setpriority(guest, a[0], a[1], a[2]),
        np::PERSONALITY => call::personality(a[0]),
        _ => return None,
    })
}
