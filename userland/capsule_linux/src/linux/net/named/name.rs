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

//! Unix names. A path is a file, as on Linux: bind makes it, it stays after
//! the socket closes until the guest unlinks it, and a connect to a path
//! with no listener is ECONNREFUSED if the file is there and ENOENT if not.
//! An abstract name has no file and goes with its socket. Neither is seen
//! outside the family: only the family's sockets are bound to them.

use crate::linux::abi::errno;
use crate::linux::file;
use crate::linux::guest::Guest;

use super::addr::UAddr;
use crate::linux::net::sock::{self, Domain, Proto, UName};

/// The name `ua` means for this guest; None asks for one to be chosen.
pub fn resolve(guest: &Guest, ua: &UAddr) -> Option<UName> {
    match ua {
        UAddr::Auto => None,
        UAddr::Path(p) => Some(UName { key: file::visible(&guest.cwd, p), shown: p.clone() }),
        UAddr::Abstract(n) => {
            let mut key = alloc::vec![0u8];
            key.extend_from_slice(n);
            Some(UName { key: key.clone(), shown: key })
        }
    }
}

/// The family socket of kind `proto` bound to `name`, the one a connect or
/// a send reaches: a stream's must listen. Otherwise Linux's answer.
pub fn find(name: &UName, proto: Proto) -> Result<u32, i64> {
    /*
     * An accepted connection carries its listener's name, as on Linux; the
     * listener is the one a connect reaches.
     */
    let found = sock::with(|t| {
        let named =
            || t.iter().filter(|(_, s)| s.domain == Domain::Unix && s.uname.as_ref() == Some(name));
        named()
            .find(|(_, s)| s.listening)
            .or_else(|| named().next())
            .map(|(i, s)| (i, s.proto, s.listening))
    });
    match found {
        Some((_, p, _)) if p != proto => Err(errno::EPROTOTYPE),
        Some((i, Proto::Dgram, _)) | Some((i, _, true)) => Ok(i),
        Some(_) => Err(errno::ECONNREFUSED),
        None if name.is_abstract() || file::look(&name.key).is_some() => Err(errno::ECONNREFUSED),
        None => Err(errno::ENOENT),
    }
}
