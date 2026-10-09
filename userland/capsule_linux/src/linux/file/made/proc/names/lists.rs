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

/* The names each /proc directory holds. */

/* Linux's system-wide files that NONOS declares. */
pub const SYSTEM: [&[u8]; 7] =
    [b"cpuinfo", b"filesystems", b"loadavg", b"meminfo", b"stat", b"uptime", b"version"];

/* Each process's files; `mem` is listed, as on Linux, and refused at open. */
pub const FILES: [&[u8]; 12] = [
    b"cgroup",
    b"cmdline",
    b"comm",
    b"environ",
    b"limits",
    b"maps",
    b"mem",
    b"mountinfo",
    b"mounts",
    b"stat",
    b"statm",
    b"status",
];

pub(super) const LINKS: [&[u8]; 3] = [b"cwd", b"exe", b"root"];

pub(super) const DIRS: [&[u8]; 3] = [b"fd", b"fdinfo", b"task"];
