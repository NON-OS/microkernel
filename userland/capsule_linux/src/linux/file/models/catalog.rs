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
 * What a user chooses from: /models lists every pinned model, and
 * /models/tiers says each one's tier, length and digest, one a line, from
 * the signed table. Neither is on the volume, and opening them marks
 * nothing: only a model marks the family.
 */

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest};

use super::super::flags::{wants_read, O_CREAT};
use super::super::{desc, slot};
use super::name::ROOT;
use super::pinned::PINNED;

pub const TIERS: &[u8] = b"/models/tiers";

/* "small qwen2.5-0.5b-instruct-q4_k_m.gguf 491400032 74a4...", a line each. */
pub fn text() -> Vec<u8> {
    let mut out = String::new();
    for p in PINNED {
        let name = core::str::from_utf8(&p.name[1..]).unwrap_or("?");
        let _ = write!(out, "{} {} {} ", p.tier, name, p.bytes);
        for b in p.sha256 {
            let _ = write!(out, "{b:02x}");
        }
        out.push('\n');
    }
    out.into_bytes()
}

/* Open /models or /models/tiers; None for any other path. */
pub fn open(guest: &mut Guest, path: &[u8], flags: u64) -> Option<u64> {
    let fd = if path == ROOT {
        let mut names: Vec<String> = [".", "..", "tiers"].map(String::from).to_vec();
        names.extend(PINNED.iter().filter_map(|p| String::from_utf8(p.name[1..].to_vec()).ok()));
        let mut fd = Fd::dir(path.to_vec(), names);
        fd.handle = desc::fresh(false, false);
        fd
    } else if path == TIERS {
        let mut fd = Fd::file(path.to_vec(), text().len() as u64, None, false);
        fd.handle = desc::fresh(false, wants_read(flags));
        fd
    } else {
        return None;
    };
    if flags & 3 != 0 || flags & O_CREAT != 0 {
        return Some(errno::fail(errno::EROFS));
    }
    Some(match slot::install(guest, fd) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    })
}
