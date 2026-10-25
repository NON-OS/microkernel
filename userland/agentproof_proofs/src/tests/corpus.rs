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

//! The pieces the sweeps build from, and the policies they hold to.

/// What the sweep's files are built of.
#[rustfmt::skip]
pub const PIECES: &[&[u8]] = &[
    b"nonos-box", b" 1", b"\n", b"base", b" strict", b" offline", b" folders", b"write", b" /tmp",
    b" /home", b" /root", b" /run", b" /var/tmp", b" /dev/shm", b"/a", b"/b", b"/..", b"/.", b"/",
    b"network", b" none", b" any", b"connect", b"#", b" ", b"\r", b"\t", b"\0", b"\xff", b",",
    b":", b"2", b"base strict\n", b"base offline\n", b"base folders\n", b"write /tmp/a\n",
    b"write /home/w\n", b"network none\n", b"\nnetwork none\n",
];

/// What the paths a write is tried at are built of.
pub const PATHS: &[&str] = &[
    "/tmp", "/home", "/root", "/run", "/var/tmp", "/dev/shm", "/etc", "/usr", "/tmpx", "/a",
    "/out", "/work", "/b", "/", "/x/y",
];

/// Policies of every base, as their files are written.
pub const CORPUS: &[&[&str]] = &[
    &["nonos-box 1", "base strict", "network none"],
    &["nonos-box 1", "base offline", "network none"],
    &["nonos-box 1", "base folders", "write /tmp/out", "network none"],
    &["nonos-box 1", "base folders", "write /home/work", "write /tmp/out", "network none"],
    &["nonos-box 1", "base folders", "write /var/tmp", "write /dev/shm/a", "network none"],
    &["nonos-box 1", "base folders", "write /root", "write /run/b", "network none"],
    &["nonos-box 1", "base folders", "write /tmp", "write /tmp/a", "network none"],
];
