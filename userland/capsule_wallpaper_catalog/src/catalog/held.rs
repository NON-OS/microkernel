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

//! The one wallpaper the catalog holds, read out of the collection when it
//! is asked for and let go once its last chunk has been served.
//!
//! The collection is a store entry vfs streams from the device and never
//! loads (nonos_disk_map::STREAMED_PREFIX), so a wallpaper nobody asks for
//! is never read: the ones a person did not keep at setup are not offered,
//! and do not come into the session at all. An installed disk that kept
//! only some carries each of those alone instead (nonos_wallpapers), and
//! that file is read when the disk has no collection.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::VfsStream;
use nonos_wallpapers::{file_path, Pin, COLLECTION, PINS};
use spin::Mutex;

struct Held {
    index: u32,
    bytes: Vec<u8>,
}

static HELD: Mutex<Option<Held>> = Mutex::new(None);

/// Why a wallpaper could not be served.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fetch {
    /// No wallpaper has that index.
    Unknown,
    /// The store could not be read, or its bytes are not the pinned ones.
    Unavailable,
}

/// Run `f` on the bytes of wallpaper `index`, reading them first if another
/// one (or none) is held.
pub fn with_bytes<R>(index: u32, f: impl FnOnce(&[u8]) -> R) -> Result<R, Fetch> {
    let mut held = HELD.lock();
    if held.as_ref().map(|h| h.index) != Some(index) {
        // The old one goes first, so two are never held at once.
        *held = None;
        *held = Some(Held { index, bytes: read(index)? });
    }
    Ok(f(&held.as_ref().ok_or(Fetch::Unavailable)?.bytes))
}

/// Let the held wallpaper go: its last chunk is served.
pub fn release(index: u32) {
    let mut held = HELD.lock();
    if held.as_ref().map(|h| h.index) == Some(index) {
        *held = None;
    }
}

fn read(index: u32) -> Result<Vec<u8>, Fetch> {
    let pin = PINS.get(index as usize).ok_or(Fetch::Unknown)?;
    let me = nonos_libc::mk_getpid();
    let refuse = |why: &str| {
        super::say::refused(index, pin.slug, why);
        Fetch::Unavailable
    };
    let bytes = match VfsStream::open(me, COLLECTION.as_bytes()) {
        Ok(mut stream) => stream.read_window(pin.offset as u64, pin.len),
        Err(_) => alone(me, pin),
    }
    .map_err(refuse)?;
    if bytes.len() != pin.len as usize {
        return Err(refuse("the store gave fewer bytes than its pin"));
    }
    if nonos_hash::sha256(&bytes) != pin.sha256 {
        return Err(refuse("its bytes are not its pinned SHA-256"));
    }
    Ok(bytes)
}

/* The wallpaper as its own file, as an install that kept only some carries it. */
fn alone(me: u32, pin: &Pin) -> Result<Vec<u8>, &'static str> {
    VfsStream::open(me, file_path(pin).as_bytes())?.read_window(0, pin.len)
}
