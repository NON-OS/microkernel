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

//! `bind` on a Unix socket: a path, which becomes a file as on Linux, an
//! abstract name, or family alone, which asks for a name to be chosen.

use crate::linux::abi::errno;
use crate::linux::file;
use crate::linux::guest::Guest;

use super::addr::{unix_addr, UAddr};
use super::auto::fresh;
use super::name::resolve;
use crate::linux::net::sock;

pub fn bind(guest: &Guest, id: u32, at: u64, len: u64) -> u64 {
    let ua = match unix_addr(guest, at, len) {
        Ok(ua) => ua,
        Err(e) => return e,
    };
    if sock::with(|t| t.get(id).is_none_or(|s| s.uname.is_some())) {
        return errno::fail(errno::EINVAL);
    }
    bind_name(guest, id, &ua)
}

/// Bind Unix socket `id` to `ua`. A path must not exist yet, and becomes an
/// empty file; family alone chooses an abstract name of five hex digits.
fn bind_name(guest: &Guest, id: u32, ua: &UAddr) -> u64 {
    let name = match resolve(guest, ua) {
        Some(n) => n,
        None => match fresh() {
            Some(n) => n,
            None => return errno::fail(errno::EADDRINUSE),
        },
    };
    let taken = sock::with(|t| t.iter().any(|(_, s)| s.uname.as_ref() == Some(&name)));
    if taken || (!name.is_abstract() && file::look(&name.key).is_some()) {
        return errno::fail(errno::EADDRINUSE);
    }
    if !name.is_abstract() {
        let key = file::key(&name.key);
        if key.writable().is_err() {
            return errno::fail(errno::EROFS);
        }
        /* The store refuses a file whose directory is missing. */
        if file::store_write(&key, &[]).is_err() {
            return errno::fail(errno::ENOENT);
        }
    }
    sock::with(|t| t.get_mut(id).map(|s| s.uname = Some(name)));
    errno::ok(0)
}
