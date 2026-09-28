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

//! A program the store image names to run when nothing else was asked for.
//!
//! A test image packs its guest and a one-line file naming it, so a boot runs
//! that guest rather than the built-in busybox. Naming a program grants
//! nothing: it is read from the store like any other, so it must carry a
//! proof that verifies, and a file naming an unproven one runs nothing.

use alloc::vec::Vec;

use crate::linux::file::{key, store_read, visible};
use crate::linux::say::say;

/// Guest-visible, so it lives under /linux like the program it names.
const BOOT_GUEST: &[u8] = b"/etc/nonos-boot-guest";

const MAX_NAME: u32 = 1024;

/// The path the image names, the program's bytes and its arguments, or None
/// when the image names nothing. One argument a line, so a script passed to
/// `sh -c` needs no quoting rules.
pub(super) fn boot_guest(max_image: u32) -> Option<(Vec<u8>, Vec<u8>, Vec<Vec<u8>>)> {
    let named = read_when_ready()?;
    let mut lines = named.split(|b| *b == b'\n').filter(|l| !l.is_empty());
    let path = lines.next()?;
    if path.first() != Some(&b'/') {
        return None;
    }
    let args = lines.map(|l| l.to_vec()).collect();
    let at = visible(b"/", path);
    let bytes = store_read(&key(&at), max_image).ok()?;
    Some((at, bytes, args))
}

// The file's bytes, or None once a settled store says it has none. Until the
// VFS has finished loading the store from disk, a missing file may only be
// not loaded yet: on SMP this ran before staging and took busybox instead.
fn read_when_ready() -> Option<Vec<u8>> {
    if !super::settle::wait_settled() {
        return None;
    }
    match store_read(&key(BOOT_GUEST), MAX_NAME) {
        Ok(named) => Some(named),
        Err("vfs open failed") => None,
        Err(_) => {
            say(b"[LINUX] boot guest unreadable: store did not answer\n");
            None
        }
    }
}
