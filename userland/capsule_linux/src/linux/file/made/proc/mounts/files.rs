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

/* mounts, mountinfo and filesystems, written from the table. */

use alloc::vec::Vec;

use super::table::{of, MOUNTS};

pub fn mounts() -> Vec<u8> {
    let mut out = alloc::string::String::new();
    for (_, src, point, kind, opts, _) in MOUNTS {
        out.push_str(&alloc::format!("{src} {point} {kind} {opts} 0 0\n"));
    }
    out.into_bytes()
}

pub fn mountinfo() -> Vec<u8> {
    let mut out = alloc::string::String::new();
    for (id, src, point, kind, opts, _) in MOUNTS {
        let parent = if id == 1 { 1 } else { of(parent_of(point)).0 };
        let flags = if opts.starts_with("ro") { "ro" } else { "rw" };
        out.push_str(&alloc::format!(
            "{id} {parent} 0:{id} / {point} {opts} - {kind} {src} {flags}\n"
        ));
    }
    out.into_bytes()
}

fn parent_of(point: &str) -> &[u8] {
    let p = point.as_bytes();
    let cut = p.iter().rposition(|b| *b == b'/').unwrap_or(0);
    if cut == 0 {
        b"/"
    } else {
        &p[..cut]
    }
}

pub fn filesystems() -> Vec<u8> {
    b"nodev\tsysfs\nnodev\ttmpfs\nnodev\tdevtmpfs\nnodev\tproc\n\tnonos\n".to_vec()
}
