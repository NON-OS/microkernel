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

//! The wallpapers kept at setup, and only those. The running system's store
//! holds the whole collection; the new disk gets the collection whole when
//! every wallpaper was kept, and otherwise each kept one alone, cut out of
//! it and held to its pinned SHA-256 first, so a wallpaper not kept never
//! reaches the new disk and one the device changed is not written there.

use alloc::vec::Vec;

use nonos_policy_proto::wallpapers_kept::{kept, ALL};
use nonos_wallpapers::{file_path, COLLECTION, PINS};

use super::source::CarrySource;
use crate::store::StoreBuilder;

/// Wallpapers carried, and those kept that could not be.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Wallpapers {
    pub carried: usize,
    pub left_out: usize,
}

pub(super) fn carry_wallpapers(
    src: &mut dyn CarrySource,
    store: &mut StoreBuilder,
    set: u64,
) -> Wallpapers {
    let wanted = PINS.len();
    let whole = src.read(COLLECTION);
    if set == ALL {
        if let Some(bytes) = whole.as_ref().filter(|b| verified_all(b)) {
            if store.add_all(&[(COLLECTION, bytes)]).is_ok() {
                return Wallpapers { carried: wanted, left_out: 0 };
            }
        }
    }
    let mut out = Wallpapers::default();
    for (i, pin) in PINS.iter().enumerate() {
        if !kept(set, i as u8) {
            continue;
        }
        let path = file_path(pin);
        // From the collection, or, on a disk an install already wrote, from
        // the wallpaper's own file.
        let bytes: Option<Vec<u8>> = match &whole {
            Some(all) => all.get(pin.offset as usize..(pin.offset + pin.len) as usize).map(Vec::from),
            None => src.read(&path),
        };
        match bytes.filter(|b| holds(b, i)) {
            Some(b) if store.add_all(&[(path.as_str(), &b)]).is_ok() => out.carried += 1,
            _ => out.left_out += 1,
        }
    }
    out
}

/* The bytes are wallpaper `i`'s, by its pin. */
fn holds(bytes: &[u8], i: usize) -> bool {
    let pin = &PINS[i];
    bytes.len() == pin.len as usize && nonos_hash::sha256(bytes) == pin.sha256
}

/* The collection whole, every wallpaper in it its pinned bytes. */
fn verified_all(all: &[u8]) -> bool {
    PINS.iter().enumerate().all(|(i, p)| {
        all.get(p.offset as usize..(p.offset + p.len) as usize).is_some_and(|b| holds(b, i))
    }) && PINS.last().is_some_and(|p| all.len() == (p.offset + p.len) as usize)
}
