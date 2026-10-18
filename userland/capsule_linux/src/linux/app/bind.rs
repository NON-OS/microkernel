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

//! Binding a proved program to its policy.

use alloc::vec::Vec;

use crate::linux::attest_paths::beside;
use crate::linux::file::{key, store_found, store_read};

use super::choose::choose;
use super::identity::AppIdentity;
use nonos_linux_policy::vendor::{by_program, TABLE};

/// One byte past a policy, so a longer file is read long and refused.
const MAX_POLICY: u32 = nonos_linux_policy::LEN as u32 + 1;
/// A trailer is 72 bytes; anything near this is not one.
const MAX_TRAILER: u32 = 4096;
/// Guests hold no capabilities, so neither does what vouches for them.
const GUEST_CAPS: u64 = 0;

/// The identity of the program at `path`, whose `image` has just been
/// proved. A policy file beside it counts only if this machine vouched for
/// it; one that is there and not vouched for refuses the program. Only a
/// vendor row carries a destination list today; a market policy naming one
/// is served without NET_ANON until its list travels with it.
pub fn bind(path: &[u8], image: &[u8]) -> Result<AppIdentity, &'static str> {
    let file = vouched(path)?;
    let program = *blake3::hash(image).as_bytes();
    let list = by_program(&program).map(|row| row.dests);
    choose(&program, TABLE, file.as_deref()).map(|policy| AppIdentity::bound(policy, list))
}

/// A program out of this capsule's own image. No file stands beside it and
/// the vendor table does not name it: it runs in the owner's default box.
pub fn bind_built_in(image: &[u8]) -> AppIdentity {
    AppIdentity::built_in(*blake3::hash(image).as_bytes())
}

fn vouched(path: &[u8]) -> Result<Option<Vec<u8>>, &'static str> {
    let at = key(&beside(path, b".nonos_linux_policy.bin"));
    /* Only the store saying "not there" falls back to the default. */
    if !store_found(&at).map_err(|_| "its policy could not be looked for")? {
        return Ok(None);
    }
    let policy = store_read(&at, MAX_POLICY).map_err(|_| "its policy could not be read")?;
    let trailer =
        store_read(&key(&beside(path, b".nonos_linux_policy.zk_trailer.bin")), MAX_TRAILER)
            .map_err(|_| "its policy is not vouched for")?;
    if !nonos_libc::mk_local_verify(&policy, GUEST_CAPS, &trailer) {
        return Err("its policy is not this machine's");
    }
    Ok(Some(policy))
}
