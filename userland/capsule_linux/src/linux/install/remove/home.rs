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

//! An uninstall's last step: the package's home (file/private/home.rs)
//! goes with it, so nothing of an app the person removed stays behind and
//! a reinstall starts with an empty home. Found before its files go, from
//! the policy beside its program, or the program itself for one with none.

use nonos_linux_policy::decode;

use super::super::market::minted::{beside, POLICY};
use crate::linux::file::{key, store_read};

const MAX_IMAGE: u32 = 64 << 20;
const MAX_POLICY: u32 = nonos_linux_policy::LEN as u32 + 1;

/// The key of the home `pkg`'s program runs with, if it records one.
pub(super) fn home_of(pkg: &str) -> Option<[u8; 32]> {
    let path = super::super::recorded(pkg)?;
    let policy = store_read(&key(&beside(&path, POLICY)), MAX_POLICY).ok();
    match policy.as_deref().map(decode) {
        Some(Ok(p)) => Some(crate::linux::app::home_key(&p.listing_id, &p.program_blake3)),
        // A program bound to no file of its own runs in the unlisted home.
        _ => {
            let image = store_read(&key(&path), MAX_IMAGE).ok()?;
            Some(crate::linux::app::home_key(&[0; 32], blake3::hash(&image).as_bytes()))
        }
    }
}

/// True when the home is gone, or there was none.
pub(super) fn erase(home: Option<[u8; 32]>) -> bool {
    home.is_none_or(|k| crate::linux::file::erase_home(&k))
}
