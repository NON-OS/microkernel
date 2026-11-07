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

//! Reading the keymap out of the store, once.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ptr::null_mut;
use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering::{AcqRel, Acquire};

use nonos_libc::mk_debug;

use crate::linux::file::{key, store_read};
use crate::linux::whole::{ask, whole};

/// Where the keymap is, as a guest sees it: under the family's root, which
/// is the store's /linux, as /etc/nonos-links is. Compiled by
/// tools/nonos-linux-keymap and packed by linux_userland/Userland.mk.
const PATH: &[u8] = b"/usr/share/nonos/keymap.xkb";

/// An xkb keymap is tens of kilobytes; anything far larger is not one.
const MAX: u32 = 1 << 20;

/// The keymap as first read and extended, kept for every keyboard after:
/// the store's copy is shipped with the system and never changes while it
/// runs, and reading and extending it again cost each app's start a store
/// read of tens of kilobytes and a pass over every keycode. Set once from
/// null to a leaked box, and never freed or written through after.
static KEPT: AtomicPtr<Vec<u8>> = AtomicPtr::new(null_mut());

/// The keymap a keyboard is sent: the kept one, or the store's, read and
/// kept. A read that finds none is not kept, so the next keyboard asks the
/// store again.
pub fn load() -> Option<Vec<u8>> {
    let kept = KEPT.load(Acquire);
    if !kept.is_null() {
        // SAFETY: a non-null KEPT is only ever a pointer from Box::into_raw
        // below, published once by the compare_exchange and never freed or
        // written through, so it is a valid shared reference from then on.
        return Some(unsafe { &*kept }.clone());
    }
    let bytes = read()?;
    let raw = Box::into_raw(Box::new(bytes.clone()));
    if KEPT.compare_exchange(null_mut(), raw, AcqRel, Acquire).is_err() {
        // SAFETY: `raw` came from Box::into_raw just above and was not
        // published, since another call kept its keymap first, so this is
        // its only owner.
        drop(unsafe { Box::from_raw(raw) });
    }
    Some(bytes)
}

fn read() -> Option<Vec<u8>> {
    // A keymap past MAX is refused, never cut into one xkb cannot read.
    match store_read(&key(PATH), ask(MAX)).map(|b| whole(b, MAX)) {
        Ok(Some(bytes)) if !bytes.is_empty() => Some(with_extra(bytes)),
        _ => {
            let line = b"[WAYLAND] no keymap in the store, keys will carry no symbols\n";
            let _ = mk_debug(line.as_ptr(), line.len());
            None
        }
    }
}

/// The keymap with a key for each character the US one has none for
/// (keymap_extra.rs); the shipped one alone, said, if it cannot be added to.
fn with_extra(bytes: Vec<u8>) -> Vec<u8> {
    super::keymap_extra::extend(&bytes).unwrap_or_else(|| {
        let line = b"[WAYLAND] keymap not laid out as expected: no keys beyond US\n";
        let _ = mk_debug(line.as_ptr(), line.len());
        bytes
    })
}
