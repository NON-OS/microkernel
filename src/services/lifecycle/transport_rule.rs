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

//! When a transport lock may be taken over from its holder (L77e): only a
//! holder that never runs again, killed or gone from the table and on no
//! CPU's tables, so the guard on its stack is never dropped; never a kernel
//! caller (pid 0), which nothing kills. Pure, so it is checked on the host.

pub(crate) const fn takes_over(pid: u32, ended: bool, on_cpu: bool) -> bool {
    pid != 0 && ended && !on_cpu
}
