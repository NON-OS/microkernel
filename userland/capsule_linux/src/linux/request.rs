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

use nonos_libc::mk_args;

/// Matches the buffer the program path is read into.
const MAX_ARGS: usize = 256;

/// `install <name> <hash>` asks this capsule to fetch a package rather than
/// run a program. The hash is the market's BLAKE3 of the package the user
/// chose, as hex; a request without one is not an install request.
pub fn install_request() -> Option<(String, [u8; 32])> {
    let mut buf = [0u8; MAX_ARGS];
    let n = mk_args(buf.as_mut_ptr(), buf.len());
    if n <= 0 {
        return None;
    }
    let mut parts = buf.get(..n as usize)?.split(|b| *b == 0);
    if parts.next()? != b"install" {
        return None;
    }
    let name = parts.next().filter(|s| !s.is_empty())?;
    let hex = parts.next()?;
    if hex.len() != 64 {
        return None;
    }
    let mut pin = [0u8; 32];
    for (slot, pair) in pin.iter_mut().zip(hex.chunks(2)) {
        let s = core::str::from_utf8(pair).ok()?;
        *slot = u8::from_str_radix(s, 16).ok()?;
    }
    Some((String::from(core::str::from_utf8(name).ok()?), pin))
}
