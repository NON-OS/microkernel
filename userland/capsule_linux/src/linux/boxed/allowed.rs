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
use nonos_linux_policy::mask::{grants, NET_ANON};

use super::preset::BoxPolicy;

pub fn allowed(policy: &BoxPolicy) -> Vec<String> {
    let mut out = Vec::new();
    let folders: Vec<String> =
        policy.folders.iter().map(|f| String::from_utf8_lossy(f).into_owned()).collect();
    out.push(alloc::format!("write {}", folders.join(" ")));
    out.push("read the image, read-only".to_string());
    match grants(policy.mask, NET_ANON) {
        true => out.push(alloc::format!("network {}", dests(&policy.dests).join(" "))),
        false => out.push("network none".to_string()),
    }
    out.push("execve, each program held to this box".to_string());
    out
}

fn dests(list: &[u8]) -> Vec<String> {
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
