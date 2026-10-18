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

//! Reading what this capsule was asked to do.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_libc::mk_args;

use super::run_mode::{parse, Mode};

/// `uninstall <name>` asks this capsule to take out what package `name`
/// installed.
pub fn uninstall_request() -> Option<String> {
    let args = args()?;
    let mut parts = args.split(|b| *b == 0);
    if parts.next()? != b"uninstall" {
        return None;
    }
    let name = parts.next().filter(|s| !s.is_empty())?;
    Some(String::from(core::str::from_utf8(name).ok()?))
}

/// `run <name> [cli]` asks this capsule to start what package `name`
/// installed, in its window or, with `cli`, on the terminal that asked.
pub fn run_request() -> Option<(String, Mode)> {
    parse(&args()?)
}

/// The whole argument block, sized by asking the kernel first: an install's
/// m alone is 640 bytes.
pub(super) fn args() -> Option<Vec<u8>> {
    let len = usize::try_from(mk_args(core::ptr::null_mut(), 0)).ok()?;
    let mut buf = alloc::vec![0u8; len];
    let n = usize::try_from(mk_args(buf.as_mut_ptr(), buf.len())).ok()?;
    buf.truncate(n.min(len));
    Some(buf)
}
