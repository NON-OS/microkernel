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

//! The command names a shell user types first: shell and files.

/// The shell itself, what it is, and what runs on it.
pub(super) const SHELL: &[&[u8]] = &[
    b"help",
    b"about",
    b"bench",
    b"version",
    b"receipt",
    b"log",
    b"whoami",
    b"caps",
    b"capsules",
    b"clear",
    b"display",
    b"echo",
    b"history",
    b"market",
    b"motd",
    b"ping",
    b"svc",
    b"service",
    b"nox",
    b"exit",
];

/// Moving around the filesystem and changing what is in it.
pub(super) const FILES: &[&[u8]] = &[
    b"where",
    b"pwd",
    b"in",
    b"cd",
    b"ls",
    b"dir",
    b"read",
    b"cat",
    b"write",
    b"copy",
    b"cp",
    b"mk",
    b"mkdir",
    b"rm",
    b"del",
    b"mv",
    b"move",
    b"stat",
    b"find",
    b"du",
    b"touch",
    b"basename",
    b"dirname",
];
