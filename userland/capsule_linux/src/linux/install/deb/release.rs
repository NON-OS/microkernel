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

//! A suite's Release file: the SHA-256 of every index it vouches for.
//!
//! Valid-Until is not checked: this machine has no clock it trusts for it,
//! so a replayed older Release is not caught here. It is said, not hidden.

use alloc::string::String;
use alloc::vec::Vec;

use super::super::hex::hex32;

pub struct Sum {
    pub sha256: [u8; 32],
    pub size: usize,
    pub path: String,
}

pub fn sums(text: &str) -> Vec<Sum> {
    let mut out = Vec::new();
    let mut in_sha = false;
    for line in text.lines() {
        if !line.starts_with(' ') {
            in_sha = line.trim_end() == "SHA256:";
            continue;
        }
        let f: Vec<&str> = line.split_whitespace().collect();
        if let (true, [hex, size, path]) = (in_sha, f.as_slice()) {
            if let (Some(sha256), Ok(size)) = (hex32(hex), size.parse()) {
                out.push(Sum { sha256, size, path: String::from(*path) });
            }
        }
    }
    out
}
