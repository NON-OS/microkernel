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

//! Which program to run.

use alloc::vec::Vec;

use nonos_libc::mk_args;

use crate::linux::file::family::choose;
use crate::linux::file::{key, store_read, visible};

use super::launch::Launch;
use super::origin::Origin;

const MAX_IMAGE: u32 = 64 << 20;
const MAX_ARGS: usize = 256;

/// The program, where it came from, and what it is given. A run starts a
/// shipped tier, or an installed package's recorded program, or nothing:
/// the built-in program would start something the person did not ask for.
pub fn source() -> Option<Launch> {
    let store = |path: Vec<u8>, bytes, args| Launch { path, bytes, origin: Origin::Store, args };
    if let Some(name) = super::request::run_request() {
        let pkg = choose(&name);
        if let Some((path, bytes, args)) = super::install::launch(pkg) {
            return Some(store(path, bytes, args));
        }
        let path = super::install::recorded(pkg)?;
        let bytes = store_read(&key(&path), MAX_IMAGE).ok()?;
        return Some(store(path, bytes, Vec::new()));
    }
    if let Some((path, bytes)) = named() {
        return Some(store(path, bytes, Vec::new()));
    }
    if let Some((path, bytes, args)) = super::boot_guest::boot_guest(MAX_IMAGE) {
        return Some(store(path, bytes, args));
    }
    Some(super::built_in::built_in())
}

fn named() -> Option<(Vec<u8>, Vec<u8>)> {
    let mut buf = [0u8; MAX_ARGS];
    let n = mk_args(buf.as_mut_ptr(), buf.len());
    if n <= 0 {
        return None;
    }
    // The first argument is the path.
    let args = &buf[..n as usize];
    let end = args.iter().position(|b| *b == 0 || *b == b' ').unwrap_or(args.len());
    let path = args.get(..end)?;
    if path.is_empty() {
        return None;
    }
    /*
     * The argument is not a guest's, but the program it names is a
     * Linux one and lives where Linux programs live.
     */
    let at = visible(b"/", path);
    let bytes = store_read(&key(&at), MAX_IMAGE).ok()?;
    Some((at, bytes))
}
