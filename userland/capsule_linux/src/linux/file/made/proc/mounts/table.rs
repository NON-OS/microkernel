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

/* The mount table, and the mount a path is on. */

/* Mount id, source, mount point, type, options, statfs magic. */
pub const MOUNTS: [(u32, &str, &str, &str, &str, u64); 10] = [
    (1, "nonos", "/", "nonos", "ro,relatime", 0x6E6F_6E6F),
    (2, "devtmpfs", "/dev", "devtmpfs", "ro,nosuid,relatime", 0x0102_1994),
    (3, "proc", "/proc", "proc", "ro,nosuid,nodev,noexec,relatime", 0x9FA0),
    (4, "sysfs", "/sys", "sysfs", "ro,nosuid,nodev,noexec,relatime", 0x6265_6572),
    (5, "tmpfs", "/dev/shm", "tmpfs", "rw,nosuid,nodev", 0x0102_1994),
    (6, "tmpfs", "/home", "tmpfs", "rw,nosuid,nodev", 0x0102_1994),
    (7, "tmpfs", "/root", "tmpfs", "rw,nosuid,nodev", 0x0102_1994),
    (8, "tmpfs", "/run", "tmpfs", "rw,nosuid,nodev", 0x0102_1994),
    (9, "tmpfs", "/tmp", "tmpfs", "rw,nosuid,nodev", 0x0102_1994),
    (10, "tmpfs", "/var/tmp", "tmpfs", "rw,nosuid,nodev", 0x0102_1994),
];

/* The mount `path` is on: the longest mount point that contains it. */
pub fn of(path: &[u8]) -> (u32, &'static str, u64) {
    let within = |point: &str| {
        let p = point.as_bytes();
        p == b"/" || (path.starts_with(p) && matches!(path.get(p.len()), None | Some(b'/')))
    };
    let best = MOUNTS.iter().filter(|m| within(m.2)).max_by_key(|m| m.2.len());
    best.map_or((1, "nonos", 0x6E6F_6E6F), |m| (m.0, m.3, m.5))
}

/*
 * st_dev for a file on mount `id`: an anonymous device, 0:id, as Linux
 * gives every filesystem with no block device.
 */
pub fn dev(id: u32) -> u64 {
    u64::from(id)
}
