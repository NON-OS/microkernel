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
 * Everything a guest can learn about the system it runs on, in one place.
 *
 * uname, /proc, /sys and sysinfo all answer from these values and from
 * nothing else. Each is what NONOS declares to a Linux family, never a fact
 * of the machine underneath: the host's CPU model, memory size, boot id or
 * name never reach a guest, so no two families can tell they share a host.
 */

mod names;
mod sizes;

pub use names::{DOMAIN, HOSTNAME, MACHINE, OSTYPE, RELEASE, VERSION};
pub use sizes::{CPUS, HPAGE_PMD, HZ, MEMORY, OVERCOMMIT, PID_MAX, PIPE_MAX};
