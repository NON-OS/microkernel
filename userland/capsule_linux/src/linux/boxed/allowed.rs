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

//! What a box allowed, a line each, as its report says it.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use nonos_linux_policy::dests::{entries, Kind};
use nonos_linux_policy::mask::{grants, EXEC, FILES_HOME, NET_ANON};

use super::folders::under;
use super::preset::BoxPolicy;

const HOMES: &[&[u8]] = &[b"/home", b"/root"];

/// What the program was given, a line each: the box as it narrowed the
/// program's own grants (`granted`), or the box as named if it never started.
pub fn allowed(policy: &BoxPolicy, granted: Option<u64>) -> Vec<String> {
    let mask = granted.unwrap_or(policy.mask);
    // /home and /root are the run's own only with FILES_HOME (file/private/rule.rs).
    let folders: Vec<String> = policy
        .folders
        .iter()
        .filter(|f| grants(mask, FILES_HOME) || !HOMES.iter().any(|h| under(f, h)))
        .map(|f| String::from_utf8_lossy(f).into_owned())
        .collect();
    let mut out = Vec::new();
    out.push(match folders.is_empty() {
        true => "write nothing".to_string(),
        false => alloc::format!("write {}", folders.join(" ")),
    });
    out.push("read the image, read-only".to_string());
    out.push(match grants(mask, NET_ANON) && !policy.dests.is_empty() {
        true => alloc::format!("network {}", dests(&policy.dests).join(" ")),
        false => "network none".to_string(),
    });
    out.push(match grants(mask, EXEC) {
        true => "execve, each program held to this box".to_string(),
        false => "execve none".to_string(),
    });
    out
}

pub fn dests(list: &[u8]) -> Vec<String> {
    entries(list)
        .map(|d| {
            let host = match d.kind {
                Kind::V4 => {
                    alloc::format!("{}.{}.{}.{}", d.body[0], d.body[1], d.body[2], d.body[3])
                }
                _ => String::from_utf8_lossy(d.body).into_owned(),
            };
            alloc::format!("{host}:{}", d.first_port)
        })
        .collect()
}
