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

//! The installed packages' paths (member/held_map.rs), read from their
//! records once an install, at its first member check, and kept as this
//! install writes and takes records out, so each check asks by path.

use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

use nonos_app_skeleton::clients::vfs;
use nonos_libc::mk_getpid;

use super::super::manifest::Manifest;
use super::super::member::held_map::Held;
use super::one::{dir, read};

struct Paths(RefCell<Option<Held>>);

// SAFETY: an install runs on this capsule's one thread; nothing else here reaches it.
unsafe impl Sync for Paths {}

static PATHS: Paths = Paths(RefCell::new(None));

/// The first of `mine` another installed package holds, or Err when a
/// record cannot be read whole: its files might be shared, so nothing is
/// placed on a guess.
pub(in super::super) fn held_by_another(
    mine: &[Vec<u8>],
    name: &str,
) -> Result<Option<Vec<u8>>, ()> {
    let mut paths = PATHS.0.borrow_mut();
    if paths.is_none() {
        let listed = vfs::list_paths(mk_getpid(), &dir()).unwrap_or_default();
        let names = listed.iter().map(|p| p.rsplit('/').next().unwrap_or(p)).map(String::from);
        let mut held = Held::default();
        for n in names.filter(|n| !n.is_empty()) {
            held.keep(&n, &read(&n).ok_or(())?);
        }
        *paths = Some(held);
    }
    Ok(paths.as_ref().and_then(|h| h.held_by_another(mine, name)).map(<[u8]>::to_vec))
}

/// `name`'s record as just written, or gone (empty), in the paths read.
pub(in super::super) fn kept(name: &str, m: &Manifest) {
    if let Some(held) = PATHS.0.borrow_mut().as_mut() {
        held.keep(name, m);
    }
}
